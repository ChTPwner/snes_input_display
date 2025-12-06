mod button;
mod button_map;
pub mod skin;
mod theme;

use crate::skins::button::Button;
use crate::skins::button_map::ButtonsMap;
use crate::skins::theme::Theme;

use ggez::Context;
use quick_xml::{
    events::{BytesStart, Event},
    reader::Reader,
};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
    error::Error,
    fs,
    io::{self, Read},
    path::Path,
    path::PathBuf,
};

use crate::controller::pressed::Pressed;

type LayoutResult = Result<(Vec<Theme>, BTreeMap<Pressed, Button>), Box<dyn Error>>;
type AttributeResult = Result<HashMap<String, String>, Box<dyn Error>>;

fn get_layout(file_path: PathBuf, name: &str, ctx: &mut Context) -> LayoutResult {
    let file = load_file(&file_path)?;
    // let layout_name = Path::new(name);
    let mut reader = Reader::from_str(&file);
    let mut _metadata: HashMap<String, String> = HashMap::new();
    let mut backgrounds: Vec<Theme> = Vec::new();
    let mut buttons: BTreeMap<Pressed, Button> = BTreeMap::new();

    loop {
        match reader.read_event() {
            Ok(Event::Empty(t)) => match t.name().as_ref() {
                b"background" => {
                    let bg = Theme::new(t, name, ctx)?;
                    backgrounds.push(bg);
                }
                b"button" => {
                    let bt = Button::new(t, name, ctx)?;
                    buttons.insert(bt.name, bt);
                }
                _ => {}
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    Ok((backgrounds, buttons))
}

fn load_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn parse_backgrounds(backgrounds_vec: Vec<Theme>, theme: &String) -> Option<Theme> {
    backgrounds_vec
        .into_iter()
        .find(|background| background.theme.eq(theme))
}

/// Generic helper that builds a fixed-size array of items for the expected `Pressed` ordering.
/// This allows testing the mapping logic using simple types (e.g. integers) without constructing
/// heavy `Button` values.
fn buttons_map_to_array_generic<T>(
    buttons_map: BTreeMap<Pressed, T>,
) -> Result<[T; 12], Box<dyn Error>> {
    // The expected ordering (index -> Pressed) used by `ButtonsMap::index` implementation.
    const ORDER: [Pressed; 12] = [
        Pressed::R,
        Pressed::L,
        Pressed::X,
        Pressed::A,
        Pressed::Right,
        Pressed::Left,
        Pressed::Down,
        Pressed::Up,
        Pressed::Start,
        Pressed::Select,
        Pressed::Y,
        Pressed::B,
    ];

    // Collect items in ORDER, producing an io::Error if any are missing.
    // Use a simple loop instead of try_fold to avoid type-inference ambiguity and keep the logic explicit.
    let mut map = buttons_map;
    let mut vec: Vec<T> = Vec::with_capacity(12);
    for key in &ORDER {
        if let Some(item) = map.remove(key) {
            vec.push(item);
        } else {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Missing button: {:?}", key),
            )));
        }
    }

    // Convert Vec<T> -> [T; 12], returning a descriptive error if the length doesn't match.
    let arr: [T; 12] = vec.try_into().map_err(|v: Vec<T>| -> Box<dyn Error> {
        Box::from(format!("Expected 12 items, got {}", v.len()))
    })?;

    Ok(arr)
}

/// Produces an owned, boxed `ButtonsMap` from a `BTreeMap<Pressed, Button>`. Delegates to the
/// generic builder above.
fn buttons_map_to_array(
    buttons_map: BTreeMap<Pressed, Button>,
) -> Result<Box<ButtonsMap>, Box<dyn Error>> {
    let arr = buttons_map_to_array_generic(buttons_map)?;
    Ok(Box::new(ButtonsMap(arr)))
}

fn parse_attributes(t: BytesStart) -> AttributeResult {
    let mut attributes_map = HashMap::new();
    for attr in t.attributes().with_checks(false) {
        let attr = attr.map_err(|e| Box::<dyn Error>::from(e))?;
        let key_bytes = attr.key.local_name().into_inner();
        let key = std::str::from_utf8(key_bytes)?.to_string();
        let value = attr.unescape_value()?.into_owned();
        attributes_map.insert(key, value);
    }
    Ok(attributes_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn buttons_map_to_array_missing_returns_err() {
        let map: BTreeMap<Pressed, i32> = BTreeMap::new();
        let res = buttons_map_to_array_generic(map);
        assert!(res.is_err(), "Expected error when buttons are missing");
    }

    #[test]
    fn buttons_map_to_array_complete_succeeds_and_preserves_order() {
        let mut map: BTreeMap<Pressed, i32> = BTreeMap::new();

        // Insert values in arbitrary order; the function should reorder them into the expected array order.
        map.insert(Pressed::A, 3);
        map.insert(Pressed::B, 11);
        map.insert(Pressed::X, 2);
        map.insert(Pressed::Y, 10);
        map.insert(Pressed::L, 1);
        map.insert(Pressed::R, 0);
        map.insert(Pressed::Left, 5);
        map.insert(Pressed::Right, 4);
        map.insert(Pressed::Up, 7);
        map.insert(Pressed::Down, 6);
        map.insert(Pressed::Start, 8);
        map.insert(Pressed::Select, 9);

        let arr = buttons_map_to_array_generic(map).expect("should succeed with full map");

        // Verify the array matches the expected ORDER mapping (index -> value)
        for (idx, &val) in arr.iter().enumerate() {
            assert_eq!(
                val as usize, idx,
                "array value at index {} should be {}",
                idx, idx
            );
        }
    }
}
