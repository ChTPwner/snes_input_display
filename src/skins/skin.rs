use crate::skins::ButtonsMap;
use crate::skins::Theme;
use crate::skins::{buttons_map_to_array, get_layout, parse_backgrounds};
use ggez::Context;

use std::{error::Error, path::Path, path::PathBuf};

// #[derive(Debug)]
pub struct Skin {
    // pub metadata: HashMap<String, String>,
    pub background: Theme,
    pub buttons: Box<ButtonsMap>,
    pub directory: PathBuf,
    pub name: String,
    pub theme: String,
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
        let directory = file_path.parent().unwrap().to_owned();

        let (backgrounds, buttons) = get_layout(file_path, name, ctx)?;
        let background = parse_backgrounds(backgrounds, theme).unwrap();
        Ok(Self {
            // metadata,
            background,
            buttons: buttons_map_to_array(buttons),
            directory,
            name: name.to_owned(),
            theme: theme.to_owned(),
        })
    }
}
