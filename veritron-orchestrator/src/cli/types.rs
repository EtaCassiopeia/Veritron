use clap::{Args, Parser, Subcommand};

use super::helper::{DESCRIPTION, LOGO};

#[derive(Parser, Debug)]
#[command(author, version, name = "veritron")]
#[command(
about = format!("{} \n {}", LOGO, DESCRIPTION),
long_about = None,
)]
pub struct Cli {
    #[arg(global = true, short = 'c', long, default_value = "./config.yaml")]
    pub config_path: String,

    #[clap(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Start the Veritron Orchestrator. This will start the API and App servers.")]
    App(App),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct App {
    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Debug, Subcommand)]
pub enum AppCommands {
    Run,
}
