use serde::{Deserialize, Deserializer};

pub mod button_state;
pub mod buttons_iter;
pub mod controller;
pub mod pressed;
use crate::controller::button_state::ButtonState;

/// Serialization function for converting a 24-bit hex address string into `u32`.
fn hex_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let hex_address = String::deserialize(deserializer)?;
    u32::from_str_radix(&hex_address, 16).map_err(Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::pressed::Pressed;

    #[test]
    pub fn test_buttons_iter() {
        let mut some_buttons_iter = ButtonState::from_le_bytes([0xA0, 0x03]).iter();
        assert_eq!(Some(Pressed::Left), some_buttons_iter.next());
        assert_eq!(Some(Pressed::Right), some_buttons_iter.next());
        assert_eq!(Some(Pressed::A), some_buttons_iter.next());
        assert_eq!(Some(Pressed::L), some_buttons_iter.next());
        assert_eq!(None, some_buttons_iter.next());

        let mut no_buttons_iter = ButtonState::from_le_bytes([0x00, 0x00]).iter();
        assert_eq!(None, no_buttons_iter.next());
    }
}
