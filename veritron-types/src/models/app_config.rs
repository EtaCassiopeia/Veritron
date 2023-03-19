use crate::constants::DEFAULT_HOME_DIR;
use serde::{
    de::{self, IgnoredAny, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, PartialEq, Eq, Clone, Default, Debug)]
/// The configuration for the app
pub struct Config {
    /// name of the app
    #[serde(default = "Veritron")]
    pub app_name: String,
    #[serde(default = "default_home_dir")]
    ///directory for all process; Default: ~/.veritron
    pub home_dir: String,
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

                while let Some(key) = access.next_key()? {
                    match key {
                        "app_name" => {
                            if app_name.is_some() {
                                return Err(de::Error::duplicate_field("app_name"));
                            }
                            app_name = Some(access.next_value()?);
                        }
                        "home_dir" => {
                            if home_dir.is_some() {
                                return Err(de::Error::duplicate_field("home_dir"));
                            }
                            home_dir = Some(access.next_value()?);
                        }
                        _ => {
                            let _: IgnoredAny = access.next_value()?;
                        }
                    }
                }

                let app_name = app_name.unwrap_or_else(|| "Veritron".to_owned());
                let home_dir = home_dir.unwrap_or_else(default_home_dir);

                Ok(Config { app_name, home_dir })
            }
        }

        deserializer.deserialize_map(ConfigVisitor)
    }
}
