use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;

use crate::{
    outbound,
    config,
    outbound::user_api::{ApiClient, GpuRequirement, ResourceRequirements, SubmitJobRequest},
    domain::agent::daemon::Daemon,
};

pub async fn start_agent(config: config::AgentConfig) -> Result<()> {
    println!("Initializing Lilac agent...");

    // 1. Initialize all the adapters.
    let control_plane_client = outbound::control_plane::ControlPlaneClient::new(config.clone());
    let system_monitor = outbound::system::HybridMonitor::new();
    let docker_executor = outbound::docker::DockerExecutor::new(config.clone())?;

    // 2. Initialize and run the daemon.
    let daemon = Daemon::new(
        control_plane_client,
        system_monitor,
        docker_executor,
        config.node_id,
    );

    daemon.run().await?;
    Ok(())
}

pub async fn configure_user(config: config::UserConfig) -> Result<()> {
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

    let toml_string = toml::to_string(&new_config)?;
    let config_path = config::get_config_path("config.toml")?;
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(config_path, toml_string)?;

    println!("✅ User configuration saved successfully.");
    Ok(())
}

pub async fn configure_agent(config: config::AgentConfig) -> Result<()> {
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

    let toml_string = toml::to_string(&new_config)?;
    let config_path = config::get_config_path("agent.toml")?;
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(config_path, toml_string)?;

    println!("✅ Agent configuration saved successfully.");
    Ok(())
}

pub async fn submit_job(config: config::UserConfig) -> Result<()> {
    let theme = ColorfulTheme::default();
    let client = ApiClient::new(config.clone());

    let name: String = Input::with_theme(&theme)
        .with_prompt("What would you like to name this job?")
        .interact_text()?;

    let docker_uri: String = Input::with_theme(&theme)
        .with_prompt("What is the Docker image URI for your job?")
        .interact_text()?;

    let queues = client.get_queues().await?;
    let queue_names: Vec<String> = queues.iter().map(|q| q.name.clone()).collect();

    let queue_selection = Select::with_theme(&theme)
        .with_prompt("Which queue should this job be submitted to?")
        .items(&queue_names)
        .default(0)
        .interact()?;

    let selected_queue = &queues[queue_selection];

    let requested_cpu: i32 = Input::with_theme(&theme)
        .with_prompt("How much CPU do you need (in millicores)?")
        .interact_text()?;

    let requested_memory: i32 = Input::with_theme(&theme)
        .with_prompt("How much Memory (in MB)?")
        .interact_text()?;

    let gpu_required = Confirm::with_theme(&theme)
        .with_prompt("Do you require GPUs?")
        .default(false)
        .interact()?;

    let mut gpu_count: Option<i32> = None;
    let mut gpu_model: Option<String> = None;
    let mut gpu_memory_gb: Option<i32> = None;

    if gpu_required {
        gpu_count = Some(
            Input::with_theme(&theme)
                .with_prompt("How many GPUs?")
                .interact_text()?,
        );

        gpu_model = Some(
            Input::with_theme(&theme)
                .with_prompt("What model of GPU? (e.g., A100, V100, leave blank for any)")
                .allow_empty(true)
                .interact_text()?,
        );
        if gpu_model.as_deref() == Some("") {
            gpu_model = None;
        }

        let memory_input: String = Input::with_theme(&theme)
            .with_prompt("What is the minimum memory per GPU (in GB)? (leave blank for any)")
            .allow_empty(true)
            .interact_text()?;

        if !memory_input.is_empty() {
            gpu_memory_gb = Some(memory_input.parse()?);
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
        let model_str = gpu_model.as_deref().unwrap_or("any");
        let memory_str = gpu_memory_gb.map_or("".to_string(), |mem| format!(" ({}GB VRAM)", mem));
        println!("- GPUs: {} x {}{}", count, model_str, memory_str);
    }

    if !Confirm::with_theme(&theme)
        .with_prompt("Proceed with job submission?")
        .default(true)
        .interact()?
    {
        println!("Submission cancelled.");
        return Ok(());
    }

    println!("\n📨 Submitting job to the Lilac scheduler...");
    let gpus = if let Some(count) = gpu_count {
        Some(GpuRequirement {
            count,
            model: gpu_model,
            memory_gb: gpu_memory_gb,
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