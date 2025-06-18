use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[arg(short, long)]
        name: Option<String>,
    },
    Add {
        #[arg(short, long)]
        feature: Option<String>,
    },
    List,
}
