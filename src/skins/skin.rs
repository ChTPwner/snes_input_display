use crate::skins::ButtonsMap;
use crate::skins::Theme;
use crate::skins::{buttons_map_to_array, get_layout, parse_backgrounds};
use ggez::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use std::{error::Error, path::Path};

#[derive(Deserialize, Serialize, Debug)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_theme: String,
}

// #[derive(Debug)]
pub struct Skin {
    // pub metadata: HashMap<String, String>,
    pub background: Theme,
    pub buttons: Box<ButtonsMap>,
    // pub directory: PathBuf,
    // pub name: String,
    // pub theme: String,
}

impl Skin {
    pub fn new(
        path: &Path,
        name: &String,
        theme: &String,
        ctx: &mut Context,
    ) -> Result<Skin, Box<dyn Error>> {
        let skin_filename = "skin.xml";
        let file_path = path.join(name).join(skin_filename);

        let (backgrounds, buttons) = get_layout(file_path, name, ctx)?;
        let background = match parse_backgrounds(backgrounds, theme) {
            Some(t) => t,
            None => return Err("could not parse background".into()),
        };
        Ok(Self {
            background,
            buttons: buttons_map_to_array(buttons),
        })
    }
}
