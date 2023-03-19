use crate::errors::OrchestrationError;
use crate::Orchestrator;
use veritron_types::models::app_config::Config;

#[derive(Default, Clone)]
pub struct SimpleOrchestrator {
    pub config: Config,
}

impl SimpleOrchestrator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Orchestrator for SimpleOrchestrator {
    fn run(&self) -> Result<(), OrchestrationError> {
        // todo!()
        println!("running");
        Ok(())
    }
}
