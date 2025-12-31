use rusb2snes::USB2SnesEndpoint;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_to_string, write, File};
use std::path::{Path, PathBuf};

use crate::controller::controller_impl::ControllerConfig;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppConfig {
    pub controller: ControllerConfig,
    pub skin: SkinConfig,
    pub usb2snes: Option<USB2SnesEndpoint>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_background: Option<String>,
}

impl AppConfig {
    pub fn new(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        // compute config_file_path
        let config_dir_path = match dirs::config_local_dir() {
            Some(c) => c,
            None => return Err("Can't figure out configuration directory".into()),
        };

        let config_file_path = match path {
            Some(p) => PathBuf::from(p),
            None => config_dir_path
                .join("snes-input-display")
                .join("settings.toml"),
        };

        let config_file_path = match config_file_path.to_str() {
            Some(s) => s,
            None => return Err("Cannot compute the configuration file path".into()),
        };

        dbg!(config_file_path);

        // check if path exists or create default settings file
        if !Path::new(&config_file_path).exists() {
            Self::create_default(config_file_path)?;
        }

        // read and load config
        let contents = read_to_string(config_file_path)?;
        let config: AppConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    fn create_default(path: &str) -> Result<(), Box<dyn Error>> {
        println!("Creating a new settings file: {path}");
        let documents_dir = match dirs::document_dir() {
            Some(p) => p,
            None => return Err("Could not compute Documents directory".into()),
        };
        let default_dir = documents_dir.join("snes-input-display");
        let default_inputs_file_path = default_dir.join("inputs_addresses.json");
        let default_skins_dir_path = default_dir.join("skins");

        let config = AppConfig {
            controller: ControllerConfig {
                input_config_path: default_inputs_file_path,
                layout: "Default".to_string(),
            },
            skin: SkinConfig {
                skins_path: default_skins_dir_path,
                skin_name: "skin_folder_name".to_string(),
                skin_background: Some("skin_theme".to_string()),
            },
            usb2snes: Some(USB2SnesEndpoint::default()),
        };
        let toml = toml::to_string(&config)?;
        File::create(path)?;
        write(path, toml)?;
        Ok(())
    }
}
