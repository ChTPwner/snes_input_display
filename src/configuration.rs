// use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_to_string, write, File};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerConfig {
    pub input_config_path: PathBuf,
    pub layout: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_theme: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub controller: ControllerConfig,
    pub skin: SkinConfig,
}

impl AppConfig {
    pub fn new(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        let config_file_path = match path {
            Some(p) => PathBuf::from(p),
            None => dirs::config_local_dir()
                .unwrap()
                .join("snes-input-display")
                .join("settings.toml"),
        };
        let config_file_path = config_file_path.to_str().unwrap();
        dbg!(config_file_path);
        if !Path::new(&config_file_path).exists() {
            Self::create_default(config_file_path)?;
        }
        // let mut file = File::open(config_file_path)?;
        let contents = read_to_string(config_file_path)?;
        let config: AppConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    fn create_default(path: &str) -> Result<(), Box<dyn Error>> {
        println!("Creating a new settings file: {path}");
        let default_dir = dirs::document_dir().unwrap().join("snes-input-display");
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
                skin_theme: "skin_theme".to_string(),
            },
        };
        let toml = toml::to_string(&config)?;
        File::create(path)?;
        write(path, toml)?;
        Ok(())
    }
}
