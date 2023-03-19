use serde::{
    de::{self, IgnoredAny, Visitor},
    Deserialize, Deserializer,
};

use crate::constants::DEFAULT_HOME_DIR;
use crate::models::api_config::default_api_config;
use crate::models::api_config::ApiConfig;
use crate::models::api_endpoint::ApiEndpoint;
use crate::models::connection::Connection;
use crate::models::source::Source;

#[derive(PartialEq, Eq, Clone, Default, Debug)]
/// The configuration for the app
pub struct Config {
    /// name of the app
    pub app_name: String,
    ///directory for all process; Default: ~/.veritron
    pub home_dir: String,

    /// Api server config related: port, host, etc
    pub api: Option<ApiConfig>,
    pub connections: Vec<Connection>,
    /// sources to ingest data related to particular connection
    pub sources: Vec<Source>,
    /// api endpoints to expose
    pub endpoints: Vec<ApiEndpoint>,
}

pub fn default_home_dir() -> String {
    DEFAULT_HOME_DIR.to_owned()
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfigVisitor;

        impl<'de> Visitor<'de> for ConfigVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Veritron Config")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut app_name = None;
                let mut home_dir = None;
                let mut api: Option<ApiConfig> = Some(default_api_config());
                let mut connections: Vec<Connection> = vec![];
                let mut sources_value: Vec<serde_yaml::Value> = vec![];
                let mut endpoints_value: Vec<serde_yaml::Value> = vec![];

                while let Some(key) = access.next_key()? {
                    match key {
                        "app_name" => {
                            if app_name.is_some() {
                                return Err(de::Error::duplicate_field("app_name"));
                            }
                            app_name = Some(access.next_value()?);
                            //set_field("app_name", &mut app_name, access.next_value)?;
                        }
                        "home_dir" => {
                            if home_dir.is_some() {
                                return Err(de::Error::duplicate_field("home_dir"));
                            }
                            home_dir = Some(access.next_value()?);
                        }
                        "api" => {
                            api = Some(access.next_value::<ApiConfig>()?);
                        }
                        "connections" => {
                            connections = access.next_value::<Vec<Connection>>()?;
                        }
                        "sources" => {
                            sources_value = access.next_value::<Vec<serde_yaml::Value>>()?;
                        }
                        "endpoints" => {
                            endpoints_value = access.next_value::<Vec<serde_yaml::Value>>()?;
                        }
                        _ => {
                            let _: IgnoredAny = access.next_value()?;
                        }
                    }
                }

                let app_name = app_name.unwrap_or_else(|| "Veritron".to_owned());
                let home_dir = home_dir.unwrap_or_else(default_home_dir);

                let result_sources: Result<Vec<Source>, A::Error> = sources_value
                    .iter()
                    .enumerate()
                    .map(|(idx, source_value)| -> Result<Source, A::Error> {
                        let connection_ref = source_value["connection"].to_owned();
                        if connection_ref.is_null() {
                            return Err(de::Error::custom(format!(
                                "sources[{idx:}]: missing connection ref"
                            )));
                        }
                        let connection_ref: super::source::Value = serde_yaml::from_value(
                            source_value["connection"].to_owned(),
                        )
                            .map_err(|err| {
                                de::Error::custom(format!(
                                    "sources[{idx:}]: connection ref - {err:} "
                                ))
                            })?;
                        let super::source::Value::Ref(connection_name) = connection_ref;
                        let mut source: Source = serde_yaml::from_value(source_value.to_owned())
                            .map_err(|err| {
                                de::Error::custom(format!("sources[{idx:}]: {err:} "))
                            })?;
                        let connection = connections
                            .iter()
                            .find(|c| c.name == connection_name)
                            .ok_or_else(|| {
                                de::Error::custom(format!(
                                    "sources[{idx:}]: Cannot find Ref connection name: {connection_name:}"
                                ))
                            })?;
                        source.connection = Some(connection.to_owned());
                        Ok(source)
                    })
                    .collect();

                let sources = result_sources?;

                let endpoints = endpoints_value
                    .iter()
                    .enumerate()
                    .map(|(idx, endpoint_value)| -> Result<ApiEndpoint, A::Error> {
                        let endpoint: ApiEndpoint =
                            serde_yaml::from_value(endpoint_value.to_owned()).map_err(|err| {
                                de::Error::custom(format!("api_endpoints[{idx:}]: {err:} "))
                            })?;
                        Ok(endpoint)
                    })
                    .collect::<Result<Vec<ApiEndpoint>, A::Error>>()?;

                Ok(Config {
                    app_name,
                    home_dir,
                    api,
                    connections,
                    sources,
                    endpoints,
                })
            }
        }

        deserializer.deserialize_map(ConfigVisitor)
    }
}

//TODO fix this: `api_config::_::_serde::de::Error` cannot be made into an object
// fn set_field<T,F>(field: &str, value: &mut Option<T>, load_value:F) -> Result<(), dyn de::Error>
// where
//     F: FnOnce() -> T,
// {
//     match value {
//         Some(_) => Err(de::Error::duplicate_field(field)),
//         None => {
//             *value = Some(load_value());
//             Ok(())
//         }
//     }
// }
