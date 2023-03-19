use prettytable::Table;
use serde::Deserialize;

use crate::ingestion_types::LocalStorage;

#[derive(Deserialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct Connection {
    pub config: Option<ConnectionConfig>,
    pub name: String,
}

#[derive(Deserialize, Eq, PartialEq, Clone, Hash, Debug)]
pub enum ConnectionConfig {
    /// In yaml, present as tag: `!ObjectStore`
    LocalStorage(LocalStorage),
}
