use clap::Parser;
use lilac::{
    config,
    errors::CliError,
    inbound::{
        cli::{AgentCommands, Cli, Commands},
        handlers,
    },
};

#[tokio::main]
pub async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Submit(args) => {
            let config = config::load_user_config()?;
            handlers::submit_job(config, args).await?;
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