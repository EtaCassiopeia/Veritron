use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ApiIndex {
    pub primary_key: Vec<String>,
}

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ApiEndpoint {
    pub name: String,
    /// name of the table in source database; Type: String
    pub table_name: String,

    /// path of endpoint - e.g: /stocks
    pub path: String,
    pub index: Option<ApiIndex>,
}
