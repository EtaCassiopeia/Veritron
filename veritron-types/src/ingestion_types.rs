use std::fmt::Debug;

use prettytable::{table, Table as PrettyTable};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::internal::BoxedError;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct Table {
    pub name: String,
    pub prefix: String,
    pub file_type: String,
    pub extension: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct LocalDetails {
    pub path: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct LocalStorage {
    pub details: Option<LocalDetails>,
    pub tables: Vec<Table>,
}

impl LocalStorage {
    pub fn convert_to_table(&self) -> PrettyTable {
        self.details
            .as_ref()
            .map_or_else(|| table!(), |details| table!(["path", details.path]))
    }
}
