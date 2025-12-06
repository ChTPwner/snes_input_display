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
    error::Error,
    fs,
    io::Read,
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

/// Produces an boxed array indexable by `Pressed` that maps a single button press to an
/// initialized `Button`.
fn buttons_map_to_array(mut buttons_map: BTreeMap<Pressed, Button>) -> Box<ButtonsMap> {
    debug_assert!(buttons_map.len() >= 12);
    let array: ButtonsMap;
    for (_, button) in buttons_map.iter() {
        button
    }
    let array = ButtonsMap([
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
        buttons_map.pop_first()?.1,
    ]);
    
    Box::new(array)
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
