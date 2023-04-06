use std::path::PathBuf;
use std::{env, fs};

use serde::{Deserialize, Serialize};

use super::common::*;
use super::errors::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub modules: Vec<Module>,
}
impl Config {
    pub fn new() -> Result<Config> {
        let mut conf = Config::default();

        // if config file not exist, create one with example
        if !Config::config_file_exist() {
            save_config(&conf)?;
        }

        Ok(conf)
    }

    pub fn config_file_path() -> PathBuf {
        let mut config_file_path = env::current_dir().unwrap();
        config_file_path.push("statusbar.json");
        config_file_path
    }

    pub fn config_file_exist() -> bool {
        Config::config_file_path().is_file()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            modules: vec![Module::default()],
        }
    }
}

pub fn load_config() -> Result<Config> {
    let setting_str = fs::read_to_string(&Config::config_file_path())?;
    let val: Config = serde_json::from_str(&setting_str)?;
    Ok(val)
}

pub fn save_config(config: &Config) -> Result<()> {
    let contents = serde_json::to_string_pretty(&config)?;
    fs::write(Config::config_file_path(), contents.as_bytes())?;
    Ok(())
}
