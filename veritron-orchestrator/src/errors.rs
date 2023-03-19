#![allow(clippy::enum_variant_names)]

use std::fmt::Display;

use veritron_types::errors::internal::BoxedError;
use veritron_types::thiserror::Error;
use veritron_types::{serde_yaml, thiserror};

#[derive(Error, Debug)]
pub enum OrchestrationError {
    CliError(#[from] CliError),
}

impl Display for OrchestrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrchestrationError::CliError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Can't find the configuration file at: {0:?}")]
    FailedToLoadFile(String),
    #[error("Failed to parse veritron config: {0:?}")]
    FailedToParseYaml(#[source] BoxedError),
}
