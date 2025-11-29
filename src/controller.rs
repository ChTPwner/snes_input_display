pub mod button_state;
pub mod buttons_iter;
pub mod controller_addresses;
pub mod controller_impl;
pub mod pressed;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::controller::{
        button_state::ButtonState,
        controller_impl::{ControllerConfig, ControllerData},
        pressed::Pressed,
    };

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

    #[test]
    pub fn test_controller_data() {
        let config = ControllerConfig {
            input_config_path: PathBuf::from("confs/Defaults.json"),
            layout: String::from("A Link To The Past"),
        };

        let mut controller_data = ControllerData::new(&config).unwrap();

        let mut expected_layouts_name = vec![
            "Default".to_string(),
            "Super Mario World".to_string(),
            "Ninja Gaiden Trilogy".to_string(),
            "A Link To The Past".to_string(),
            "Demon's Crest/Demon's Blazon".to_string(),
            "Super Metroid Emu".to_string(),
            "Mega Man X".to_string(),
            "Mega Man X2".to_string(),
        ];
        expected_layouts_name.sort();

        assert_eq!(controller_data.available_layouts, expected_layouts_name);

        let expected_low_address: u32 = 0xF500F2;
        let expected_high_address: u32 = 0xF500F0;

        assert_eq!(
            expected_low_address,
            controller_data.current_addresses.address_low
        );
        assert_eq!(
            expected_high_address,
            controller_data.current_addresses.address_high
        );

        let expected_next_index = controller_data.current_layout_index + 1;
        let expected_next_layout = controller_data.available_layouts[expected_next_index].clone();
        let expected_low_address: u32 = 0xF90718;
        let expected_high_address: u32 = 0xF90719;
        controller_data.get_next_layout();
        assert_eq!(expected_next_index, controller_data.current_layout_index);
        assert_eq!(
            expected_next_layout,
            controller_data.available_layouts[expected_next_index]
        );
        assert_eq!(
            expected_low_address,
            controller_data.current_addresses.address_low
        );
        assert_eq!(
            expected_high_address,
            controller_data.current_addresses.address_high
        );
    }
}
