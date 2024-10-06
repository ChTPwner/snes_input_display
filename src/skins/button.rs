use crate::controller::pressed::Pressed;
use crate::skins::parse_attributes;

use ggez::{
    graphics::{Image, Rect},
    Context,
};

use quick_xml::events::BytesStart;
use std::{error::Error, path::Path};

#[derive(Debug)]
pub struct Button {
    pub name: Pressed,
    pub image: Image,
    pub rect: Rect,
}

impl Button {
    pub fn new(t: BytesStart, dir: &str, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
        let attributes = parse_attributes(t);
        let x = attributes["x"].parse::<f32>().unwrap();
        let y = attributes["y"].parse::<f32>().unwrap();
        let image_path = Path::new("/").join(dir).join(&attributes["image"]);

        let image = Image::from_path(ctx, image_path)?;
        // let image_info = ImageInfo::from_file_path(&image_path)?;
        let width = image.width() as f32;
        let height = image.height() as f32;

        let name = match attributes["name"].as_str() {
            "a" => Pressed::A,
            "b" => Pressed::B,
            "x" => Pressed::X,
            "y" => Pressed::Y,
            "select" => Pressed::Select,
            "start" => Pressed::Start,
            "l" => Pressed::L,
            "r" => Pressed::R,
            "up" => Pressed::Up,
            "down" => Pressed::Down,
            "left" => Pressed::Left,
            "right" => Pressed::Right,
            _ => panic!(),
        };

        Ok(Self {
            name,
            image,
            rect: Rect::new(x, y, width, height),
        })
    }
}
