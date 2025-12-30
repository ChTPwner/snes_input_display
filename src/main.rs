#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controller;
mod input_viewer;
mod skins;
use input_viewer::{InputViewer, APP_NAME};
use std::{env, error::Error};

use ggez::{event, ContextBuilder, GameResult};

use configuration::AppConfig;

fn main() -> Result<GameResult, Box<dyn Error>> {
    /* Setup Configs */
    let config_path = env::args().nth(1);
    let app_config = AppConfig::new(config_path)?;

    let (mut ctx, event_loop) = ContextBuilder::new(APP_NAME, "ChTPwner")
        .add_resource_path(&app_config.skin.skins_path)
        .build()
        .expect("aieee, could not create ggez context!");

    let input_viewer = InputViewer::new(&mut ctx, app_config)?;
    event::run(ctx, event_loop, input_viewer)
}
