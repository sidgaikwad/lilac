use crate::{
    config::AgentConfig,
    domain::agent::{models::JobDetails, ports::JobExecutor, models::NodeResources},
    errors::JobExecutorError,
};
use async_trait::async_trait;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
    StopContainerOptions, WaitContainerOptions,
};
use bollard::image::CreateImageOptions;
use bollard::{auth::DockerCredentials, Docker};
use futures_util::stream::StreamExt;

#[derive(Clone)]
pub struct DockerExecutor {
    docker: Docker,
    config: AgentConfig,
}

impl DockerExecutor {
    pub fn new(config: AgentConfig) -> Result<Self, JobExecutorError> {
        // Connect to the local Docker daemon.
        // This will fail if Docker is not running.
        let docker =
            Docker::connect_with_local_defaults().map_err(|e| JobExecutorError::Unknown(e.into()))?;
        Ok(Self { docker, config })
    }
}

#[async_trait]
impl JobExecutor for DockerExecutor {
    async fn run_job(
        &self,
        job_details: JobDetails,
        resources: &NodeResources,
    ) -> Result<i64, JobExecutorError> {
        println!("[DOCKER] Starting job: {}", job_details.id);
        println!("[DOCKER] Pulling image: {}", job_details.docker_uri);

        // 1. Pull the Docker image.
        let credentials = if let Some(private_registry) = &self.config.private_registry {
            Some(DockerCredentials {
                serveraddress: Some(private_registry.registry_url.clone()),
                username: Some(private_registry.username.clone()),
                password: Some(private_registry.secret.clone()),
                ..Default::default()
            })
        } else {
            None
        };

        let mut stream = self.docker.create_image(
            Some(CreateImageOptions {
                from_image: job_details.docker_uri.clone(),
                ..Default::default()
            }),
            None,
            credentials,
        );

        while let Some(result) = stream.next().await {
            result.map_err(|e| JobExecutorError::Unknown(e.into()))?;
        }

        // 2. Clean up any old container with the same name, just in case.
        let container_name = format!("lilac-job-{}", job_details.id);
        let remove_options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        // We don't care if this fails, it just means the container didn't exist.
        let _ = self
            .docker
            .remove_container(&container_name, remove_options)
            .await;

        // 3. Create the container.
        let options = Some(CreateContainerOptions {
            name: container_name.clone(),
            ..Default::default()
        });

        let mut host_config = bollard::service::HostConfig {
            ..Default::default()
        };

        if !resources.gpus.is_empty() {
            host_config.device_requests = Some(vec![bollard::service::DeviceRequest {
                driver: Some("".to_string()),
                count: Some(-1),
                device_ids: None,
                capabilities: Some(vec![vec!["gpu".to_string()]]),
                options: None,
            }]);
        }

        let config = Config {
            image: Some(job_details.docker_uri.clone()),
            host_config: Some(host_config),
            ..Default::default()
        };

        let container = self
            .docker
            .create_container(options, config)
            .await
            .map_err(|e| JobExecutorError::Unknown(e.into()))?;
        println!("[DOCKER] Created container with ID: {}", container.id);

        // 4. Start the container.
        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .map_err(|e| JobExecutorError::Unknown(e.into()))?;
        println!("[DOCKER] Started container for job {}", job_details.id);

        // 5. Wait for the container to finish.
        let wait_options = Some(WaitContainerOptions {
            condition: "not-running",
        });
        let mut stream = self.docker.wait_container(&container.id, wait_options);
        let wait_result = stream.next().await.unwrap().unwrap();
        let exit_code = wait_result.status_code;
        println!(
            "[JOB {}] Execution finished with exit code: {}",
            job_details.id, exit_code
        );

        // 6. Remove the container.
        self.docker
            .remove_container(
                &container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| JobExecutorError::Unknown(e.into()))?;
        println!("[DOCKER] Removed container: {}", container_name);

        // 7. Remove the image to save space.
        let _ = self
            .docker
            .remove_image(&job_details.docker_uri, None, None)
            .await;
        println!("[DOCKER] Attempted to remove image: {}", job_details.docker_uri);


        Ok(exit_code)
    }

    async fn stop_job(&self, job_id: &str) -> Result<(), JobExecutorError> {
        let container_name = format!("lilac-job-{}", job_id);
        println!("[DOCKER] Stopping container: {}", container_name);

        // Stop the container. Give it 30 seconds to shut down gracefully.
        let stop_options = Some(StopContainerOptions { t: 30 });
        self.docker
            .stop_container(&container_name, stop_options)
            .await
            .map_err(|e| JobExecutorError::Unknown(e.into()))?;
        println!("[DOCKER] Stopped container: {}", container_name);

        // Remove the container.
        let remove_options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        self.docker
            .remove_container(&container_name, remove_options)
            .await
            .map_err(|e| {
                if !e.to_string().contains("404") {
                    eprintln!("[DOCKER] Error removing container {}: {}", container_name, e);
                }
                JobExecutorError::Unknown(e.into())
            })?;
        println!("[DOCKER] Removed container: {}", container_name);

        Ok(())
    }
}