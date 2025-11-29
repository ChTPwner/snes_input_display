use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

use crate::controller::controller_addresses::ControllerAddresses;

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerConfig {
    pub input_config_path: PathBuf,
    pub layout: String,
}

// struct ControllerLayout {
//     name: String,
//     addresses: ControllerAddresses,
// }

#[derive(Deserialize, Debug, Clone)]
pub struct ControllerLayouts {
    pub layouts: HashMap<String, ControllerAddresses>,
}

#[derive(Deserialize, Debug)]
pub struct ControllerData {
    pub available_addresses: ControllerLayouts,
    pub available_layouts: Vec<String>,
    pub current_addresses: ControllerAddresses,
    pub current_layout_index: usize,
}

impl ControllerData {
    pub fn new(config: &ControllerConfig) -> Result<Self, Box<dyn Error>> {
        // get path of layouts json from config file
        let config_data =
            fs::read_to_string(&config.input_config_path).expect("Unable open to config file");

        let available_addresses: ControllerLayouts =
            serde_json::from_str(&config_data).expect("Unable to parse");

        let mut available_layouts: Vec<String> = available_addresses
            .layouts
            .iter()
            .map(|(name, _)| name.clone())
            .collect();
        available_layouts.sort();

        let current_layout_index = available_layouts
            .iter()
            .position(|n| n == &config.layout)
            .unwrap();

        let current_addresses = available_addresses.layouts[&config.layout];

        Ok(ControllerData {
            available_addresses,
            available_layouts,
            current_layout_index,
            current_addresses,
        })
    }

    pub fn get_next_layout(&mut self) {
        // add one and modulo to loop on the list
        self.current_layout_index = (self.current_layout_index + 1) % self.available_layouts.len();
        self.current_addresses =
            self.available_addresses.layouts[&self.available_layouts[self.current_layout_index]];
    }
}
