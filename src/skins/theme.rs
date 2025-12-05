use crate::skins::parse_attributes;
use ggez::{graphics::Image, Context};
use quick_xml::events::BytesStart;
use std::{error::Error, path::Path, path::MAIN_SEPARATOR_STR};

#[derive(Debug, Clone)]
pub struct Theme {
    pub theme: String,
    pub image: Image,
    // pub width: f32,
    pub height: f32,
}

impl Theme {
    pub fn new(t: BytesStart, dir: &str, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
        let attributes = parse_attributes(t)?;
        let image_path = Path::new(MAIN_SEPARATOR_STR).join(dir).join(&attributes["image"]);
        let image = Image::from_path(ctx, image_path)?;
        // let width = image.width() as f32;
        let height = image.height() as f32;

        Ok(Self {
            theme: attributes["name"].to_owned().to_lowercase(),
            image,
            // width,
            height,
        })
    }
}
