use clap::Parser;

use lilac_cli::inbound::cli::{AgentCommands, Cli, Commands};
use lilac_cli::inbound::handlers;
use lilac_cli::config;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = config::load()?;

    match &cli.command {
        Commands::Submit => {
            handlers::submit_job(config).await?;
        }
        Commands::Configure => {
            handlers::configure_lilac(config).await?;
        }
        Commands::Agent(args) => match &args.command {
            AgentCommands::Start => {
                handlers::start_agent(config).await?;
            }
        },
    }

    Ok(())
}