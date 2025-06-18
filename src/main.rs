mod cli;
mod project_structure;
mod package_manager;
mod features;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use features::list_features;
use commands::create::create_project;
use commands::add::add_feature;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Create { name } => {
            create_project(name).await?;
        }
        Commands::Add { feature } => {
            add_feature(feature).await?;
        }
        Commands::List => {
            list_features()?;
        }
    }
    Ok(())
}
