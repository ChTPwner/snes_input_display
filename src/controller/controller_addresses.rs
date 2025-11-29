use rusb2snes::SyncClient;
use std::error::Error;

use serde::{Deserialize, Deserializer};

use crate::controller::button_state::ButtonState;
pub fn hex_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let hex_address = String::deserialize(deserializer)?;
    u32::from_str_radix(&hex_address, 16).map_err(Error::custom)
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ControllerAddresses {
    #[serde(deserialize_with = "hex_to_u32")]
    pub address_low: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub address_high: u32,
}

impl ControllerAddresses {
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
