use crate::controller::pressed::Pressed;
use crate::skins::background::Background;
use crate::skins::button::Button;
use crate::skins::ButtonsMap;
use crate::skins::{buttons_map_to_array, load_file, parse_backgrounds};
use ggez::Context;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;

use std::error::Error;

type LayoutResult = Result<(Vec<Background>, BTreeMap<Pressed, Button>), Box<dyn Error>>;

const SKIN_FILE_NAME: &str = "skin.xml";

#[derive(Deserialize, Serialize, Debug)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_background: String,
}

pub struct SkinData {
    pub current_skin: Skin,
    pub available_skins: Vec<String>,
}

impl SkinData {
    pub fn new(config: &SkinConfig, ctx: &mut Context) -> Result<SkinData, Box<dyn Error>> {
        let available_skins = SkinData::get_available_skins(&config.skins_path)?;

        Ok(SkinData {
            current_skin: Skin::new(config, ctx)?,
            available_skins,
        })
    }

    fn get_available_skins(path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
        let mut skins: Vec<String> = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let file_to_check = entry.path().join(SKIN_FILE_NAME);
                match Skin::get_layout(file_to_check, entry.file_name().to_str().unwrap(), ctx) {
                    Ok(_) => skins.push(entry.file_name().to_str().unwrap().to_string()),
                    Err(_) => continue,
                }
            }
            Ok(skins)
        }
    }
}

// #[derive(Debug)]
pub struct Skin {
    pub background: Background,
    pub buttons: Box<ButtonsMap>,
}

impl Skin {
    pub fn new(config: &SkinConfig, ctx: &mut Context) -> Result<Skin, Box<dyn Error>> {
        let file_path = config
            .skins_path
            .join(&config.skin_name)
            .join(SKIN_FILE_NAME);

        let (backgrounds, buttons) = Skin::get_layout(file_path, &config.skin_name, ctx)?;
        let background =
            match parse_backgrounds(backgrounds, &config.skin_background.to_lowercase()) {
                Some(t) => t,
                None => return Err("could not parse background".into()),
            };
        Ok(Self {
            background,
            buttons: buttons_map_to_array(buttons)?,
        })
    }

    fn get_layout(file_path: PathBuf, skin_dir_name: &str, ctx: &mut Context) -> LayoutResult {
        let file = load_file(&file_path)?;
        // let layout_name = Path::new(name);
        let mut reader = Reader::from_str(&file);
        let mut _metadata: HashMap<String, String> = HashMap::new();
        let mut backgrounds: Vec<Background> = Vec::new();
        let mut buttons: BTreeMap<Pressed, Button> = BTreeMap::new();

        loop {
            match reader.read_event() {
                Ok(Event::Empty(t)) => match t.name().as_ref() {
                    b"skin" => for attribute in t.attributes().with_checks(false) {},
                    b"background" => {
                        let bg = Background::new(t, skin_dir_name, ctx)?;
                        backgrounds.push(bg);
                    }
                    b"button" => {
                        let bt = Button::new(t, skin_dir_name, ctx)?;
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
}

struct SkinAttributes {
    name: String,
    author: String,
    r#type: String,
}
