use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::queue::{
    models::{Queue, QueueId},
    ports::{QueueRepository, QueueRepositoryError},
};

pub struct PostgresQueueRepository {
    pool: PgPool,
}

impl PostgresQueueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub struct QueueRecord {
    queue_id: uuid::Uuid,
    name: String,
    priority: i32,
    cluster_targets: Option<Vec<uuid::Uuid>>,
}

impl From<QueueRecord> for Queue {
    fn from(value: QueueRecord) -> Self {
        Self {
            id: value.queue_id.into(),
            name: value.name,
            priority: value.priority,
            cluster_targets: value
                .cluster_targets
                .unwrap_or_default()
                .into_iter()
                .map(|v| v.into())
                .collect(),
        }
    }
}

#[async_trait]
impl QueueRepository for PostgresQueueRepository {
    async fn get_all_queues_sorted(&self) -> Result<Vec<Queue>, QueueRepositoryError> {
        let records = sqlx::query_as!(
            QueueRecord,
            r#"
            SELECT
                q.queue_id,
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
        .await
        .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        let queues = records.into_iter().map(|record| record.into()).collect();

        Ok(queues)
    }
    async fn create(&self, queue: &Queue) -> Result<(), QueueRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        sqlx::query!(
            "INSERT INTO queues (queue_id, name, priority) VALUES ($1, $2, $3)",
            queue.id.0,
            queue.name,
            queue.priority
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        for (i, cluster_id) in queue.cluster_targets.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO queue_cluster_assignments (queue_id, cluster_id, \"order\") VALUES ($1, $2, $3)",
                queue.id.0,
                cluster_id.0,
                i as i32
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        Ok(())
    }

    async fn find_by_id(&self, queue_id: &QueueId) -> Result<Option<Queue>, QueueRepositoryError> {
        let record = sqlx::query_as!(
            QueueRecord,
            r#"
            SELECT
                q.queue_id,
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
            queue_id.0,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        Ok(record.map(|v| v.into()))
    }

    async fn update(&self, queue: &Queue) -> Result<(), QueueRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        sqlx::query!(
            "UPDATE queues SET name = $1, priority = $2 WHERE queue_id = $3",
            queue.name,
            queue.priority,
            queue.id.0
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        sqlx::query!(
            "DELETE FROM queue_cluster_assignments WHERE queue_id = $1",
            queue.id.0
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        for (i, cluster_id) in queue.cluster_targets.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO queue_cluster_assignments (queue_id, cluster_id, \"order\") VALUES ($1, $2, $3)",
                queue.id.0,
                cluster_id.0,
                i as i32
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        Ok(())
    }

    async fn delete(&self, queue_id: &QueueId) -> Result<(), QueueRepositoryError> {
        sqlx::query!("DELETE FROM queues WHERE queue_id = $1", queue_id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| QueueRepositoryError::Unknown(e.into()))?;

        Ok(())
    }
    async fn get_queue_by_id(&self, queue_id: &QueueId) -> Result<Queue, QueueRepositoryError> {
        let record = sqlx::query_as!(
            QueueRecord,
            r#"
            SELECT
                q.queue_id,
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
            queue_id.0,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => QueueRepositoryError::NotFound(queue_id.to_string()),
            _ => QueueRepositoryError::Unknown(err.into()),
        })?;

        Ok(record.into())
    }
}
