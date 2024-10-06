use crate::configuration::ControllerConfig;
use crate::controller::hex_to_u32;
use crate::controller::ButtonState;
use rusb2snes::SyncClient;

use serde::Deserialize;
use std::error::Error;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug, Clone)]
pub struct ControllerLayouts {
    pub layouts: HashMap<String, Controller>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Controller {
    #[serde(deserialize_with = "hex_to_u32")]
    pub address_low: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub address_high: u32,
}

impl Controller {
    pub fn new(config: &ControllerConfig) -> Self {
        let config_data =
            fs::read_to_string(&config.input_config_path).expect("Unable open to config file");
        let layouts_data: ControllerLayouts =
            serde_json::from_str(&config_data).expect("Unable to parse");
        layouts_data.layouts[&config.layout]
    }

    pub fn pushed(&self, client: &mut SyncClient) -> Result<ButtonState, Box<dyn Error>> {
        let base_address = std::cmp::min(self.address_low, self.address_high);
        let offset_low = self.address_low.saturating_sub(base_address) as usize;
        let offset_high = self.address_high.saturating_sub(base_address) as usize;
        let read_length = offset_low.abs_diff(offset_high).saturating_add(1);
        debug_assert!((2..256).contains(&read_length));
        let input_bytes = client.get_address(base_address, read_length)?;
        let button_state =
            ButtonState::from_le_bytes([input_bytes[offset_low], input_bytes[offset_high]]);

        Ok(button_state)
    }
}
