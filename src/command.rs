use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "remote-compose")]
#[command(about = "A tool for remotely deploying Docker Compose stacks", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Deploys and restarts docker stacks on remote machines
    #[command(arg_required_else_help = true)]
    Deploy {
        /// Path to the config file
        #[arg(short = 'p')]
        config_path: String
    }
}