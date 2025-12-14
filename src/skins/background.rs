use ggez::{graphics::Image, Context};
use quick_xml::events::BytesStart;
use std::{
    error::Error,
    path::{Path, MAIN_SEPARATOR_STR},
};

use crate::skins::parse_attributes;

#[derive(Debug, Clone)]
pub struct Background {
    pub name: String,
    pub image: Image,
    // pub width: f32,
    pub height: f32,
}

impl Background {
    pub fn new(t: BytesStart, skin_dir: &str, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
        let attributes = parse_attributes(t)?;
        let image_path = Path::new(MAIN_SEPARATOR_STR)
            .join(skin_dir)
            .join(&attributes["image"]);
        let image = Image::from_path(ctx, image_path)?;
        // let width = image.width() as f32;
        let height = image.height() as f32;

        Ok(Self {
            name: attributes["name"].to_owned().to_lowercase(),
            image,
            // width,
            height,
        })
    }
}
