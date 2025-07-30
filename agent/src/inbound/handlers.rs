use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use fs_extra::dir::{copy, CopyOptions};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rand_distr::Alphanumeric;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tempfile;
use tokio::process::Command;

use crate::{
    outbound,
    client::{ApiClient, GpuRequirement, ResourceRequirements, SubmitJobRequest},
    config,
    domain::agent::daemon::Daemon,
};

pub async fn start_agent(config: config::Config) -> Result<()> {
    println!("Initializing Lilac agent...");

    // 1. Initialize all the adapters.
    let control_plane_client = outbound::control_plane::ControlPlaneClient::new(config);
    let system_monitor = outbound::system::SysinfoMonitor::new();
    let docker_executor = outbound::docker::DockerExecutor::new()?;

    // 2. Initialize and run the daemon.
    let daemon = Daemon::new(
        control_plane_client,
        system_monitor,
        docker_executor,
    );

    daemon.run().await?;
    Ok(())
}

pub async fn configure_lilac(config: config::Config) -> Result<()> {
    let theme = ColorfulTheme::default();

    let api_endpoint: String = Input::with_theme(&theme)
        .with_prompt("Enter the Lilac API endpoint")
        .with_initial_text(config.api_endpoint)
        .interact_text()?;

    let api_key: String = Input::with_theme(&theme)
        .with_prompt("Enter your API key (optional)")
        .with_initial_text(config.api_key.unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;

    let container_registry_url: String = Input::with_theme(&theme)
        .with_prompt("Enter the container registry URL")
        .with_initial_text(config.container_registry_url)
        .interact_text()?;

    let base_image: String = Input::with_theme(&theme)
        .with_prompt("Enter the base image name")
        .with_initial_text(config.base_image)
        .interact_text()?;

    let new_config = config::Config {
        api_endpoint,
        container_registry_url,
        base_image,
        api_key: if api_key.is_empty() {
            None
        } else {
            Some(api_key)
        },
        node_id: config.node_id, // Preserve the existing node_id
    };

    let toml_string = toml::to_string(&new_config)?;
    let config_path = config::get_config_path()?;
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(config_path, toml_string)?;

    println!("✅ Configuration saved successfully.");
    Ok(())
}

pub async fn submit_job(config: config::Config) -> Result<()> {
    let theme = ColorfulTheme::default();
    let client = ApiClient::new(config.clone());

    let name: String = Input::with_theme(&theme)
        .with_prompt("What would you like to name this job?")
        .with_initial_text("my-training-run")
        .interact_text()?;

    let workdir: String = Input::with_theme(&theme)
        .with_prompt("What is the path to your project's code?")
        .with_initial_text("./")
        .interact_text()?;

    let setup_command: String = Input::with_theme(&theme)
        .with_prompt("Is there a setup command to run before your main script? (optional)")
        .with_initial_text("pip install -r requirements.txt")
        .allow_empty(true)
        .interact_text()?;

    let command: String = Input::with_theme(&theme)
        .with_prompt("What is the main command to run for your job?")
        .with_initial_text("python main.py")
        .interact_text()?;

    let queues = client.get_queues().await?;
    let queue_names: Vec<String> = queues.iter().map(|q| q.name.clone()).collect();

    let queue_selection = Select::with_theme(&theme)
        .with_prompt("Which queue should this job be submitted to?")
        .items(&queue_names)
        .default(0)
        .interact()?;

    let selected_queue = &queues[queue_selection];

    let cpu: i32 = Input::with_theme(&theme)
        .with_prompt("How much CPU do you need (in millicores)?")
        .with_initial_text("1000")
        .interact_text()?;

    let memory: i32 = Input::with_theme(&theme)
        .with_prompt("How much Memory (in MB)?")
        .with_initial_text("4096")
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
    println!("- Workdir: {}", workdir);
    if !setup_command.is_empty() {
        println!("- Setup Command: {}", setup_command);
    }
    println!("- Command: {}", command);
    println!(
        "- Queue: {} ({})",
        selected_queue.name, selected_queue.id
    );
    println!("- CPU: {}m", cpu);
    println!("- Memory: {}MB", memory);
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

    // --- Task 6: Visual Walkthrough & Task 5: Dockerfile Generation ---
    println!("\n[1/4] 📝 Generating ephemeral Dockerfile...");
    let image_tag: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let image_name = format!(
        "{}/{}:{}",
        config.container_registry_url, name, image_tag
    );

    // If a setup command is provided, we use a multi-stage build to keep the final image small.
    // Otherwise, a simple single-stage build is sufficient.
    let cmd_parts: Vec<&str> = command.split_whitespace().collect();
    let cmd_array = format!(
        "[{}]",
        cmd_parts
            .iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let dockerfile_content = if setup_command.is_empty() {
        format!(
            r#"
FROM {base_image}
COPY . /app
WORKDIR /app
ENTRYPOINT {cmd_array}
"#,
            base_image = config.base_image,
            cmd_array = cmd_array
        )
    } else {
        format!(
            r#"
# --- Builder Stage ---
# This stage installs dependencies into a virtual environment.
FROM {base_image} AS builder

# Create a virtual environment
RUN python -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Install dependencies
COPY requirements.txt .
RUN {setup_command}

# Copy the rest of the application code
COPY . /app


# --- Final Stage ---
# This is the final, clean image.
FROM {base_image}

# Copy the virtual environment from the builder stage
COPY --from=builder /opt/venv /opt/venv

# Copy the application code from the builder stage
COPY --from=builder /app /app

# Set the path to use the virtual environment and set the working directory
ENV PATH="/opt/venv/bin:$PATH"
WORKDIR /app
ENTRYPOINT {cmd_array}
"#,
            base_image = config.base_image,
            setup_command = setup_command,
            cmd_array = cmd_array
        )
    };
    println!("      ✅ Generated image name: {}", image_name);

    println!("[2/4] 🐳 Building and pushing Docker image...");
    let build_context = tempfile::tempdir()?;
    fs::write(
        build_context.path().join("Dockerfile"),
        dockerfile_content,
    )?;

    let workdir_path = Path::new(&workdir);
    let mut options = CopyOptions::new(); // Customize options here if needed
    options.content_only = true;
    copy(workdir_path, build_context.path(), &options)?;

    let build_child = Command::new("docker")
        .arg("buildx")
        .arg("build")
        .arg("--platform=linux/amd64,linux/arm64")
        .arg("-t")
        .arg(&image_name)
        .arg(build_context.path())
        .arg("--push")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")?
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message("Building and pushing image...");

    let build_output = build_child.wait_with_output().await?;

    pb.finish_and_clear();

    let build_status = build_output.status;
    if !build_status.success() {
        println!("❌ Docker build and push failed. See logs below:\n");
        println!("{}", String::from_utf8_lossy(&build_output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&build_output.stderr));
        anyhow::bail!("Docker build and push failed.");
    }
    println!("      ✅ Image built and pushed successfully.");

    println!("[4/4] 📨 Submitting job to the Lilac scheduler...");
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
        definition: image_name,
        queue_id: selected_queue.id.clone(),
        resource_requirements: ResourceRequirements {
            cpu_millicores: cpu,
            memory_mb: memory,
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
        Err(_) => {
            eprintln!("\n❌ Error submitting job.");
            eprintln!("Please check the following:");
            eprintln!(
                "  - Is the Lilac server running and reachable at the configured API endpoint?"
            );
            eprintln!("  - Have you configured the correct API key with `lilac-agent configure`?");
        }
    }
    Ok(())
}