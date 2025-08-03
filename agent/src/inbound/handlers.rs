use crate::{
    config,
    domain::agent::daemon::Daemon,
    errors::CliError,
    inbound::cli::SubmitArgs,
    outbound,
    outbound::user_api::{ApiClient, GpuRequirement, ResourceRequirements, SubmitJobRequest},
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;

pub async fn start_agent(config: config::AgentConfig) -> Result<(), CliError> {
    println!("Initializing Lilac agent...");

    // 1. Initialize all the adapters.
    let control_plane_client = outbound::control_plane::ControlPlaneClient::new(config.clone());
    let system_monitor = outbound::system::HybridMonitor::new();
    let docker_executor = outbound::docker::DockerExecutor::new(config.clone())
        .map_err(|e| CliError::Unknown(e.into()))?;

    // 2. Initialize and run the daemon.
    let daemon = Daemon::new(
        control_plane_client,
        system_monitor,
        docker_executor,
        config.node_id,
    );

    daemon
        .run()
        .await
        .map_err(|e| CliError::Unknown(e.into()))?;
    Ok(())
}

pub async fn configure_user(config: config::UserConfig) -> Result<(), CliError> {
    let theme = ColorfulTheme::default();

    let api_endpoint: String = Input::with_theme(&theme)
        .with_prompt("Enter the Lilac API endpoint")
        .with_initial_text(config.api_endpoint)
        .interact_text()?;

    let api_key: String = Input::with_theme(&theme)
        .with_prompt("Enter your User API key")
        .with_initial_text(config.api_key.unwrap_or_default())
        .interact_text()?;

    let new_config = config::UserConfig {
        api_endpoint,
        api_key: if api_key.is_empty() {
            None
        } else {
            Some(api_key)
        },
    };

    let toml_string =
        toml::to_string(&new_config).map_err(|e| CliError::Unknown(e.into()))?;
    let config_path =
        config::get_config_path("config.toml")?;
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(config_path, toml_string)?;

    println!("✅ User configuration saved successfully.");
    Ok(())
}

pub async fn configure_agent(config: config::AgentConfig) -> Result<(), CliError> {
    let theme = ColorfulTheme::default();

    let api_endpoint: String = Input::with_theme(&theme)
        .with_prompt("Enter the Lilac API endpoint")
        .with_initial_text(config.api_endpoint)
        .interact_text()?;

    let cluster_api_key: String = Input::with_theme(&theme)
        .with_prompt("Enter your Cluster API key")
        .with_initial_text(config.cluster_api_key)
        .interact_text()?;

    let mut new_config = config::AgentConfig {
        api_endpoint,
        cluster_api_key,
        node_id: config.node_id,
        private_registry: None,
    };

    if Confirm::with_theme(&theme)
        .with_prompt("Configure a private Docker registry?")
        .default(false)
        .interact()?
    {
        let registry_url: String = Input::with_theme(&theme)
            .with_prompt("Enter the private registry URL")
            .interact_text()?;
        let username: String = Input::with_theme(&theme)
            .with_prompt("Enter the registry username")
            .interact_text()?;
        let secret: String = Input::with_theme(&theme)
            .with_prompt("Enter the registry password or API key")
            .interact_text()?;

        new_config.private_registry = Some(config::PrivateRegistryConfig {
            registry_url,
            username,
            secret,
        });
    }

    let toml_string =
        toml::to_string(&new_config).map_err(|e| CliError::Unknown(e.into()))?;
    let config_path =
        config::get_config_path("agent.toml")?;
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(config_path, toml_string)?;

    println!("✅ Agent configuration saved successfully.");
    Ok(())
}

pub async fn submit_job(config: config::UserConfig, args: &SubmitArgs) -> Result<(), CliError> {
    if args.non_interactive
        && (args.name.is_none()
            || args.docker_uri.is_none()
            || args.queue_id.is_none()
            || args.cpu.is_none()
            || args.memory.is_none())
    {
        return Err(CliError::InvalidArguments);
    }

    let theme = ColorfulTheme::default();
    let client = ApiClient::new(config.clone());

    let name: String = if let Some(name) = &args.name {
        name.clone()
    } else {
        Input::with_theme(&theme)
            .with_prompt("What would you like to name this job?")
            .interact_text()?
    };

    let docker_uri: String = if let Some(docker_uri) = &args.docker_uri {
        docker_uri.clone()
    } else {
        Input::with_theme(&theme)
            .with_prompt("What is the Docker image URI for your job?")
            .interact_text()?
    };

    let queues = client.get_queues().await?;
    let selected_queue = if let Some(queue_id) = &args.queue_id {
        queues
            .iter()
            .find(|q| q.id.to_string() == *queue_id)
            .ok_or_else(|| CliError::InvalidArguments)?
            .clone()
    } else {
        let queue_names: Vec<String> = queues.iter().map(|q| q.name.clone()).collect();
        let queue_selection = Select::with_theme(&theme)
            .with_prompt("Which queue should this job be submitted to?")
            .items(&queue_names)
            .default(0)
            .interact()?;
        queues[queue_selection].clone()
    };

    let requested_cpu: i32 = if let Some(cpu) = args.cpu {
        cpu
    } else {
        Input::with_theme(&theme)
            .with_prompt("How much CPU do you need (in millicores)?")
            .interact_text()?
    };

    let requested_memory: i32 = if let Some(memory) = args.memory {
        memory
    } else {
        Input::with_theme(&theme)
            .with_prompt("How much Memory (in MB)?")
            .interact_text()?
    };

    let mut gpu_count: Option<i32> = args.gpu_count;

    if !args.non_interactive && gpu_count.is_none() {
        if Confirm::with_theme(&theme)
            .with_prompt("Do you require GPUs?")
            .default(false)
            .interact()?
        {
            let count: i32 = Input::with_theme(&theme)
                .with_prompt("How many GPUs?")
                .interact_text()?;
            gpu_count = Some(count);
        }
    }

    println!("\nJob Summary:");
    println!("- Name: {}", name);
    println!("- Docker Image: {}", docker_uri);
    println!(
        "- Queue: {} ({})",
        selected_queue.name, selected_queue.id
    );
    println!("- CPU: {}m", requested_cpu);
    println!("- Memory: {}MB", requested_memory);
    if let Some(count) = gpu_count {
        println!("- GPUs: {} x any", count);
    }

    if !args.non_interactive {
        if !Confirm::with_theme(&theme)
            .with_prompt("Proceed with job submission?")
            .default(true)
            .interact()?
        {
            println!("Submission cancelled.");
            return Ok(());
        }
    }

    println!("\n📨 Submitting job to the Lilac scheduler...");
    let gpus = if let Some(count) = gpu_count {
        Some(GpuRequirement {
            count,
            model: None,
            memory_gb: None,
        })
    } else {
        None
    };

    let request = SubmitJobRequest {
        name,
        definition: docker_uri,
        queue_id: selected_queue.id.clone(),
        resource_requirements: ResourceRequirements {
            cpu_millicores: requested_cpu,
            memory_mb: requested_memory,
            gpus,
        },
    };

    match client.submit_job(request).await {
        Ok(response) => {
            println!(
                "      ✅ Job submitted successfully! Job ID: {}",
                response.id
            );
        }
        Err(e) => {
            eprintln!("\n❌ Error submitting job: {}", e);
            eprintln!("\nPlease check the following:");
            eprintln!(
                "  - Is the Lilac server running and reachable at the configured API endpoint?"
            );
            eprintln!("  - Have you configured the correct API key with `lilac configure`?");
        }
    }
    Ok(())
}