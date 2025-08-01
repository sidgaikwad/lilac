use clap::Parser;

use lilac_cli::inbound::cli::{AgentCommands, Cli, Commands};
use lilac_cli::inbound::handlers;
use lilac_cli::config;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Submit => {
            let config = config::load_user_config()?;
            handlers::submit_job(config).await?;
        }
        Commands::Configure => {
            let config = config::load_user_config()?;
            handlers::configure_user(config).await?;
        }
        Commands::Agent(args) => {
            let config = config::load_agent_config()?;
            match &args.command {
                AgentCommands::Start => {
                    handlers::start_agent(config).await?;
                }
                AgentCommands::Configure => {
                    handlers::configure_agent(config).await?;
                }
            }
        }
    }

    Ok(())
}