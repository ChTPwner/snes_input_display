use crate::configuration::SkinConfig;
use crate::controller::pressed::Pressed;
use crate::skins::background::Background;
use crate::skins::button::Button;
use crate::skins::{buttons_map_to_array, get_wanted_background, load_file};
use crate::skins::{parse_attributes, ButtonsMap};
use ggez::Context;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use std::error::Error;

type LayoutResult = Result<(Vec<Background>, BTreeMap<Pressed, Button>), Box<dyn Error>>;

const SKIN_FILE_NAME: &str = "skin.xml";

pub struct SkinData {
    pub current_skin: Skin,
    pub available_skins: Vec<String>,
}

impl SkinData {
    pub fn new(config: &SkinConfig, ctx: &mut Context) -> Result<SkinData, Box<dyn Error>> {
        Ok(SkinData {
            current_skin: Skin::new(config, ctx)?,
            available_skins: SkinData::get_available_skins(&config.skins_path)?,
        })
    }

    pub fn get_available_skins(path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
        // read skin.xml file to find if type is snes
        let mut skins: Vec<String> = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let file_to_check = entry.path().join(SKIN_FILE_NAME);
                if let Ok(skin_name) = SkinData::validate_skin_file(&file_to_check) {
                    skins.push(skin_name);
                }
            }
        }
        skins.sort();
        Ok(skins)
    }

    fn validate_skin_file(file_path: &Path) -> Result<String, Box<dyn Error>> {
        let file = load_file(file_path)?;
        let mut reader = Reader::from_str(&file);
        loop {
            match reader.read_event() {
                Ok(Event::Start(t)) => {
                    if t.name().as_ref() == b"skin" {
                        let attributes = parse_attributes(t)?;
                        if attributes["type"] == "snes" {
                            return Ok(attributes["name"].to_owned().to_lowercase());
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
        }
        Err("Invalid file".into())
    }

    pub fn get_previous_skin(&mut self) -> Result<String, Box<dyn Error>> {
        // find the current skin index
        let index = match self
            .available_skins
            .iter()
            .position(|x| x == &self.current_skin.background.name)
        {
            Some(i) => i,
            None => return Err("Error getting previous skin".into()),
        };
        // set the new skin name
        Ok(self.available_skins
            [(index + self.available_skins.len() - 1) % self.available_skins.len()]
        .clone())
    }

    pub fn get_next_skin(&mut self) -> Result<String, Box<dyn Error>> {
        // find the current skin index
        let index = match self
            .available_skins
            .iter()
            .position(|x| x == &self.current_skin.background.name)
        {
            Some(i) => i,
            None => return Err("Error getting next skin".into()),
        };
        // set the new skin name
        Ok(self.available_skins[(index + 1) % self.available_skins.len()].clone())
    }
}

// #[derive(Debug)]
pub struct Skin {
    pub background: Background,
    pub buttons: Box<ButtonsMap>,
    pub available_backgrounds: Vec<String>,
}

impl Skin {
    pub fn new(config: &SkinConfig, ctx: &mut Context) -> Result<Skin, Box<dyn Error>> {
        let file_path = config
            .skins_path
            .join(&config.skin_name)
            .join(SKIN_FILE_NAME);
        let mut available_backgrounds: Vec<String> = Vec::new();
        let (backgrounds, buttons) = Skin::get_layout(file_path, &config.skin_name, ctx)?;
        for background in &backgrounds {
            available_backgrounds.push(background.name.clone());
        }
        let background =
            match get_wanted_background(backgrounds, &config.skin_background.to_lowercase()) {
                Some(t) => t,
                None => return Err("could not parse background".into()),
            };
        Ok(Self {
            background,
            buttons: buttons_map_to_array(buttons)?,
            available_backgrounds,
        })
    }

    pub fn get_next_background(&mut self) -> Result<String, Box<dyn Error>> {
        // find the current background index
        let index = match self
            .available_backgrounds
            .iter()
            .position(|x| x == &self.background.name)
        {
            Some(i) => i,
            None => return Err("could not find background".into()),
        };
        // set the new background name
        let new_name =
            self.available_backgrounds[(index + 1) % self.available_backgrounds.len()].clone();
        Ok(new_name)
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
