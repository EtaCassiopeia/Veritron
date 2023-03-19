use super::connection::Connection;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Source {
    /// name of the source - to distinguish between multiple sources; Type: String
    pub name: String,
    /// name of the table in source database; Type: String
    pub table_name: String,
    /// list of columns gonna be used in the source table; Type: String[]
    pub columns: Vec<String>,
    #[serde(skip_deserializing)]
    /// reference to pre-defined connection name - syntax: `!Ref <connection_name>`; Type: `Ref!` tag
    pub connection: Option<Connection>,
    /// name of schema source database; Type: String
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default = "default_refresh_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// setting for how to refresh the data; Default: RealTime
    pub refresh_config: Option<RefreshConfig>,
}

fn default_refresh_config() -> Option<RefreshConfig> {
    Some(RefreshConfig::default())
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum Value {
    Ref(String),
}

// pub enum SourceTypeConfig {
//     AppendOnly,
//     Overwrite,
// }

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum RefreshConfig {
    Hour { minute: u32 },
    Day { time: String },
    CronExpression { expression: String },
    RealTime(RealTimeConfig),
    Webhook { url: String }
}
impl Default for RefreshConfig {
    fn default() -> Self {
        RefreshConfig::RealTime(RealTimeConfig {})
    }
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct RealTimeConfig {}
