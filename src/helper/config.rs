use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::{fs, path::absolute, process::exit};

const DEFAULT_CONFIG_PATH: &str = "./config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub msg_path: String
}

impl Config {
    /// Default value
    pub fn default() -> Self {
        Self {
            token: String::default(),
            msg_path: String::from("msg.txt")
        }
    }

    // Checks if values in config are valid (for example if strings are not empty)
    pub fn validate(&self) -> Result<()> {
        let mut errors: Vec<&str> = Vec::new();
        // let mut warnings: Vec<&str> = Vec::new();

        if self.token == String::default() { errors.push("Token is empty, please fill it out!") }
        if self.msg_path == String::default() { errors.push("No path to the .txt file with message found!") }

        // if warnings.len() > 0 {
        //     eprintln!(
        //         "We've detected some warning(s) in your configuration. Be aware!\n{}",
        //         warnings.iter()
        //             .map(|e| format!("- {e}"))
        //             .collect::<Vec<_>>()
        //             .join("\n")
        //     );
        // }
        if errors.len() > 0 {
            return Err(
                anyhow!(
                    "We've detected some error(s) in your configuration. Please fix them!\n{}",
                    errors.iter()
                        .map(|e| format!("- {e}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            )
        }

        Ok(())
    }

    /// Loads a config from configuration path
    pub fn load(config_path: Option<String>) -> Result<Self> {
        let config_path = {
            let mut path = config_path.unwrap_or(String::from(DEFAULT_CONFIG_PATH));
            if !path.ends_with(".toml") {
                path.push_str(".toml");
            }

            absolute(&path)
                .map_or(
                    String::from(path),
                    |s| s.to_string_lossy().to_string()
            )
        };

        let exists = fs::exists(&config_path)
            .map_err(|why| anyhow!("{why}\n\nCould not check existance of a config file. Please ensure you have all needed permissions!"))?;

        if exists {
            let raw = fs::read_to_string(&config_path)?;

            let config = toml::from_str::<Self>(&raw)
                .map_err(|why| anyhow!("{why}\n\nCould not read config file! Is it valid?\n(Removing config will generate new default one)"))?;

            config.validate()?;

            Ok(config)
        } else {
            eprintln!("No config file found. Creating minimal default file at \"{config_path}\"! Please fill it out...");

            let default_raw = toml::to_string_pretty(&Config::default())?;
            fs::write(&config_path, default_raw)
                .map_err(|why| anyhow!("{why}\n\nCould not write config file! Please ensure you have all needed permissions!"))?;

            exit(1);
        }
    }
}