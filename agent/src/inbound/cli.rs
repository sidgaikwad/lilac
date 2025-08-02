use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Submit a new training job
    Submit(SubmitArgs),
    /// Configure the Lilac CLI for submitting jobs
    Configure,
    /// Commands for the Lilac agent daemon
    Agent(AgentArgs),
}

#[derive(Args, Debug)]
pub struct SubmitArgs {
    /// Name of the job
    #[arg(long)]
    pub name: Option<String>,
    /// Docker image URI for the job
    #[arg(long)]
    pub docker_uri: Option<String>,
    /// ID of the queue to submit the job to
    #[arg(long)]
    pub queue_id: Option<String>,
    /// CPU required in millicores
    #[arg(long)]
    pub cpu: Option<i32>,
    /// Memory required in MB
    #[arg(long)]
    pub memory: Option<i32>,
    /// Number of GPUs required
    #[arg(long)]
    pub gpu_count: Option<i32>,
    /// Model of GPU required (e.g. A100, V100)
    #[arg(long)]
    pub gpu_model: Option<String>,
    /// Minimum VRAM per GPU in GB
    #[arg(long)]
    pub gpu_memory: Option<i32>,
    /// Skip interactive prompts and submit directly
    #[arg(long, action)]
    pub non_interactive: bool,
}

#[derive(Args)]
pub struct AgentArgs {
    #[command(subcommand)]
    pub command: AgentCommands,
}

#[derive(Subcommand)]
pub enum AgentCommands {
    /// Start the agent daemon
    Start,
    /// Configure the Lilac agent
    Configure,
}