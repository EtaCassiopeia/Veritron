use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use clap::Parser;

use veritron_orchestrator::cli::init_veritron;
use veritron_orchestrator::cli::types::AppCommands;
use veritron_orchestrator::cli::types::Cli;
use veritron_orchestrator::cli::types::Commands;
use veritron_orchestrator::cli::LOGO;
use veritron_orchestrator::errors::OrchestrationError;
use veritron_orchestrator::set_ctrl_handler;
use veritron_orchestrator::set_panic_hook;
use veritron_orchestrator::simple::SimpleOrchestrator;
use veritron_orchestrator::Orchestrator;

mod errors;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        process::exit(1);
    }
}

fn render_logo() {
    use std::println as info;
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    info!("{LOGO}");
    info!("\nVeritron Version: {VERSION}\n");
}

fn run() -> Result<(), OrchestrationError> {
    set_panic_hook();

    let res: Result<(Cli, SimpleOrchestrator), OrchestrationError> = {
        let cli = Cli::parse();
        let veritron = init_veritron(cli.config_path.clone())?;
        Ok((cli, veritron))
    };

    let (cli, veritron) = match res {
        Ok((cli, veritron)) => (cli, veritron),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let running = Arc::new(AtomicBool::new(true));
    set_ctrl_handler(running.clone());

    println!("Running");

    if let Some(cmd) = cli.cmd {
        match cmd {
            Commands::App(apps) => match apps.command {
                AppCommands::Run => {
                    render_logo();
                    veritron.run()
                }
            },
        }
    } else {
        render_logo();
        //let mut veritron = init_veritron(cli.config_path)?;
        veritron.run()
    }
}
