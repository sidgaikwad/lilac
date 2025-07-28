use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::queue::{models::Queue, ports::QueueRepository};

pub struct PostgresQueueRepository {
    pool: PgPool,
}

impl PostgresQueueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QueueRepository for PostgresQueueRepository {
    async fn get_all_queues_sorted(&self) -> Result<Vec<Queue>, anyhow::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT
                q.queue_id as "id",
                q.name,
                q.priority,
                ARRAY_AGG(qca.cluster_id ORDER BY qca.order) FILTER (WHERE qca.cluster_id IS NOT NULL) as "cluster_targets: Vec<Uuid>"
            FROM
                queues q
            LEFT JOIN
                queue_cluster_assignments qca ON q.queue_id = qca.queue_id
            GROUP BY
                q.queue_id
            ORDER BY
                q.priority ASC;
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let queues = rows
            .into_iter()
            .map(|row| Queue {
                id: row.id,
                name: row.name,
                priority: row.priority,
                cluster_targets: row.cluster_targets.unwrap_or_default(),
            })
            .collect();

        Ok(queues)
    }
    async fn create(&self, queue: &Queue) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            "INSERT INTO queues (queue_id, name, priority) VALUES ($1, $2, $3)",
            queue.id,
            queue.name,
            queue.priority
        )
        .execute(&mut *tx)
        .await?;

        for (i, cluster_id) in queue.cluster_targets.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO queue_cluster_assignments (queue_id, cluster_id, \"order\") VALUES ($1, $2, $3)",
                queue.id,
                cluster_id,
                i as i32
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Queue>, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                q.queue_id as "id",
                q.name,
                q.priority,
                ARRAY_AGG(qca.cluster_id ORDER BY qca.order) FILTER (WHERE qca.cluster_id IS NOT NULL) as "cluster_targets: Vec<Uuid>"
            FROM
                queues q
            LEFT JOIN
                queue_cluster_assignments qca ON q.queue_id = qca.queue_id
            WHERE
                q.queue_id = $1
            GROUP BY
                q.queue_id;
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        let queue = row.map(|row| Queue {
            id: row.id,
            name: row.name,
            priority: row.priority,
            cluster_targets: row.cluster_targets.unwrap_or_default(),
        });

        Ok(queue)
    }

    async fn update(&self, queue: &Queue) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            "UPDATE queues SET name = $1, priority = $2 WHERE queue_id = $3",
            queue.name,
            queue.priority,
            queue.id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "DELETE FROM queue_cluster_assignments WHERE queue_id = $1",
            queue.id
        )
        .execute(&mut *tx)
        .await?;

        for (i, cluster_id) in queue.cluster_targets.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO queue_cluster_assignments (queue_id, cluster_id, \"order\") VALUES ($1, $2, $3)",
                queue.id,
                cluster_id,
                i as i32
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), anyhow::Error> {
        sqlx::query!("DELETE FROM queues WHERE queue_id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}