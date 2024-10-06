#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controller;
mod skins;
use controller::{ButtonState, Controller};

use ggez::{
    conf, event,
    graphics::{self, DrawParam, Rect},
    input::keyboard::{KeyCode, KeyInput},
    timer::sleep,
    Context, ContextBuilder, GameResult,
};
use rusb2snes::SyncClient;
use skins::Skin;
use std::ops::Index;
use std::{env, error::Error, ffi::OsString, time};

use configuration::AppConfig;

const APP_NAME: &str = "Snes Input Display";

// enum AppState {
//     // Menu,
//     InputViewer,
// }

struct InputViewer {
    controller: Controller,
    available_skins: Vec<String>,
    // available_themes
    skin: Skin,
    client: SyncClient,
    events: ButtonState,
    config: AppConfig,
    canvas_resized: bool,
}

impl InputViewer {
    fn new(ctx: &mut Context, config: AppConfig) -> Result<Self, Box<dyn Error>> {
        let controller = Controller::new(&config.controller);

        let available_skins = Skin::list_available_skins(&config.skin.skins_path, ctx)?;
        dbg!(&available_skins);

        let skin = Skin::new(
            &config.skin.skins_path,
            &config.skin.skin_name,
            Some(&config.skin.skin_theme.to_lowercase()),
            ctx,
        )?;

        /* Connect to USB2SNES Server */
        let mut client: SyncClient;

        // loop until connected to usb2snes
        loop {
            match SyncClient::connect() {
                Ok(s) => {
                    client = s;
                    let msg = format!("Connected to {}", &client.app_version()?);
                    println!("{}", msg);
                    break;
                }
                Err(_) => {
                    println!("Not connected to a usb2snes client");
                    sleep(time::Duration::from_secs(1));
                }
            }
        }

        client.set_name(String::from(APP_NAME))?;

        let devices: Vec<String>;
        // loop until a device is available
        loop {
            match client.list_device() {
                Ok(l) => {
                    if !l.is_empty() {
                        devices = l;
                        break;
                    }
                }
                Err(_) => println!("Error listing devices"),
            }
        }

        client.attach(&devices[0])?;
        let msg = format!("Attached to {}", &devices[0]);
        println!("{}", msg);

        // Set the window size
        // ctx.gfx.set_mode(conf::WindowMode {
        //     width: skin.background.image.width() as f32,
        //     height: skin.background.height,
        //     resizable: true,
        //     ..Default::default()
        // })?;

        Ok(Self {
            controller,
            available_skins,
            skin,
            client,
            events: ButtonState::default(),
            config,
            canvas_resized: true,
        })
    }
}

impl event::EventHandler for InputViewer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        // const DESIRED_FPS: u32 = 60;
        self.events = self.controller.pushed(&mut self.client).unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // let mut canvas =
        //     graphics::Canvas::from_image(ctx, self.skin.background.image.clone(), None);
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        if self.canvas_resized {
            ctx.gfx.set_mode(conf::WindowMode {
                width: self.skin.background.image.width() as f32,
                height: self.skin.background.height,
                resizable: true,
                ..Default::default()
            })?;

            // canvas.set_screen_coordinates(Rect::new(
            //     0 as f32,
            //     0 as f32,
            //     self.skin.background.image.width() as f32,
            //     self.skin.background.height,
            // ));
            let size = ctx.gfx.drawable_size();

            canvas.set_screen_coordinates(graphics::Rect::new(0., 0., size.0, size.1));
            self.canvas_resized = false;
        }

        canvas.draw(&self.skin.background.image, DrawParam::new());

        // Draw inputs
        self.events.iter().for_each(|event| {
            let button_image = &self.skin.buttons[event].image;
            canvas.draw(
                button_image,
                DrawParam::default().dest(self.skin.buttons[event].rect.point()),
            );
        });
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::R) => {
                // refresh available skins ?
                println!("R is pressed refreshing available skins.");
                match Skin::list_available_skins(&self.config.skin.skins_path, ctx) {
                    Ok(s) => self.available_skins = s,
                    Err(_) => println!("Couldn't refresh available skins"),
                }
            }
            Some(KeyCode::T) => {
                println!("T is pressed - Changing Theme");
                // find next Skin in available skins
                let max_available_skins_index = self.available_skins.len() - 1;
                // dbg!(&max_available_skins_index);
                if max_available_skins_index > 0 {
                    let next_skin_position = match self
                        .available_skins
                        .iter()
                        .position(|s| s == &self.config.skin.skin_name)
                    {
                        Some(i) if i == max_available_skins_index => 0,
                        Some(i) => i + 1,
                        None => 0,
                    };
                    let next_skin_name = &self.available_skins[next_skin_position];
                    // dbg!(next_skin_name);
                    // update Input viewer Skin with next one
                    self.skin =
                        Skin::new(&self.config.skin.skins_path, next_skin_name, None, ctx).unwrap();
                    // dbg!(&self.skin.theme);
                    self.canvas_resized = true;
                }
            }
            // Changing theme
            Some(KeyCode::B) => {
                println!("T is pressed - Changing Theme")
                //
            }
            // Changing Layout
            Some(KeyCode::L) => println!("L is pressed"),
            _ => (),
        }
        Ok(())
    }
}

fn main() -> Result<GameResult, Box<dyn Error>> {
    /* Setup Configs */
    let config_path = env::args().nth(1);
    let app_config = AppConfig::new(config_path)?;

    let (mut ctx, event_loop) = ContextBuilder::new(APP_NAME, "ChTBoner")
        .add_resource_path(&app_config.skin.skins_path)
        .window_setup(conf::WindowSetup::default().title(APP_NAME))
        .build()
        .expect("aieee, could not create ggez context!");

    let input_viewer = InputViewer::new(&mut ctx, app_config)?;
    event::run(ctx, event_loop, input_viewer);
}
