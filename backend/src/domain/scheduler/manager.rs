use std::{collections::HashMap, sync::Arc};

use super::ports::ComputePlatform;
use crate::domain::cluster::{models::ClusterId, ports::ClusterRepository};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("Platform not supported for cluster")]
    PlatformNotSupported,
    #[error("Cluster not found")]
    ClusterNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct ComputePlatformManager {
    plugins: HashMap<String, Arc<dyn ComputePlatform>>,
    cluster_repo: Arc<dyn ClusterRepository>,
}

impl ComputePlatformManager {
    pub fn new(cluster_repo: Arc<dyn ClusterRepository>) -> Self {
        Self {
            plugins: HashMap::new(),
            cluster_repo,
        }
    }

    pub fn register(&mut self, plugin: Arc<dyn ComputePlatform>) {
        self.plugins.insert(plugin.platform_type().to_string(), plugin);
    }

    pub async fn get_platform_for_cluster(
        &self,
        cluster_id: &Uuid,
    ) -> Result<Arc<dyn ComputePlatform>, ManagerError> {
        let cluster = self
            .cluster_repo
            .get_cluster_by_id(&ClusterId::from(*cluster_id))
            .await
            .map_err(|_| ManagerError::ClusterNotFound)?;

        self.plugins
            .get(&cluster.platform_type)
            .cloned()
            .ok_or(ManagerError::PlatformNotSupported)
    }
}