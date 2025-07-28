use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq)]
pub struct Queue {
    pub id: Uuid,
    pub name: String,
    pub priority: i32,
    #[sqlx(skip)]
    pub cluster_targets: Vec<Uuid>,
}