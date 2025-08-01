use async_trait::async_trait;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
    WaitContainerOptions,
};
use bollard::image::CreateImageOptions;
use bollard::{auth::DockerCredentials, Docker};
use futures_util::stream::StreamExt;

use crate::{
    config::AgentConfig,
    domain::agent::{models::JobDetails, ports::JobExecutor},
};

#[derive(Clone)]
pub struct DockerExecutor {
    docker: Docker,
    config: AgentConfig,
}

impl DockerExecutor {
    pub fn new(config: AgentConfig) -> anyhow::Result<Self> {
        // Connect to the local Docker daemon.
        // This will fail if Docker is not running.
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker, config })
    }
}

#[async_trait]
impl JobExecutor for DockerExecutor {
    async fn run_job(&self, job_details: JobDetails) -> anyhow::Result<i64> {
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
            let info = result?;
            if let Some(status) = info.status {
                println!("[DOCKER] Image pull status: {}", status);
            }
        }

        // 2. Create the container.
        let container_name = format!("lilac-job-{}", job_details.id);
        let options = Some(CreateContainerOptions {
            name: container_name.clone(),
            ..Default::default()
        });

        let config = Config {
            image: Some(job_details.docker_uri.clone()),
            ..Default::default()
        };

        let container = self.docker.create_container(options, config).await?;
        println!("[DOCKER] Created container with ID: {}", container.id);

        // 3. Start the container.
        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await?;
        println!("[DOCKER] Started container for job {}", job_details.id);

        // 4. Wait for the container to finish.
        let wait_options = Some(WaitContainerOptions {
            condition: "not-running",
        });
        let mut stream = self.docker.wait_container(&container.id, wait_options);
        let wait_result = stream.next().await.unwrap()?;
        let exit_code = wait_result.status_code;
        println!(
            "[JOB {}] Execution finished with exit code: {}",
            job_details.id, exit_code
        );

        // 5. Remove the container.
        self.docker
            .remove_container(
                &container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;
        println!("[DOCKER] Removed container: {}", container_name);

        Ok(exit_code)
    }
}