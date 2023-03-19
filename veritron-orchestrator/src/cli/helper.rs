use crate::errors::CliError;
use crate::simple::SimpleOrchestrator as Veritron;
use veritron_types::models::app_config::Config;
use veritron_types::serde_yaml;

use handlebars::Handlebars;
use std::{collections::BTreeMap, fs};

pub const LOGO: &str = r#"
      ___  __    ___  __   __
\  / |__  |__) |  |  |__) /  \ |\ |
 \/  |___ |  \ |  |  |  \ \__/ | \|
"#;

pub const DESCRIPTION: &str = "Single source of truth for your reference data";

pub fn init_veritron(config_path: String) -> Result<Veritron, CliError> {
    let config = load_config(config_path)?;
    dbg!("Config: {:#?}", config.clone());
    Ok(Veritron::new(config))
}

pub fn load_config(config_path: String) -> Result<Config, CliError> {
    let contents = fs::read_to_string(config_path.clone())
        .map_err(|_| CliError::FailedToLoadFile(config_path))?;

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("config", contents)
        .map_err(|e| CliError::FailedToParseYaml(Box::new(e)))?;

    let mut data = BTreeMap::new();

    for (key, value) in std::env::vars() {
        data.insert(key, value);
    }

    let config_str = handlebars
        .render("config", &data)
        .map_err(|e| CliError::FailedToParseYaml(Box::new(e)))?;

    let config: Config =
        serde_yaml::from_str(&config_str).map_err(|e| CliError::FailedToParseYaml(Box::new(e)))?;

    // Create home_dir if not exists.
    let _res = fs::create_dir_all(&config.home_dir);

    Ok(config)
}
