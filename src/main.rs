#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controller;
mod skins;
use controller::{button_state::ButtonState, controller_impl::Controller};

use ggez::{
    conf, event,
    graphics::{self, DrawParam},
    timer::sleep,
    Context, ContextBuilder, GameResult,
};
use rusb2snes::SyncClient;
use skins::skin::Skin;
use std::{error::Error, time};

use configuration::AppConfig;

const APP_NAME: &str = "Snes Input Display";

// enum AppState {
//     // Menu,
//     InputViewer,
// }

struct InputViewer {
    controller: Controller,
    skin: Skin,
    client: Option<SyncClient>,
    connected: bool,
    attached: bool,
    events: ButtonState,
}

impl InputViewer {
    fn new(ctx: &mut Context, config: AppConfig) -> Result<Self, Box<dyn Error>> {
        let controller = Controller::new(&config.controller);

        let skin = Skin::new(
            &config.skin.skins_path,
            &config.skin.skin_name,
            &config.skin.skin_theme.to_lowercase(),
            ctx,
        )?;

        /* Connect to USB2SNES Server */

        // loop until connected to usb2snes

        // Set the window size
        ctx.gfx.set_mode(conf::WindowMode {
            width: skin.background.image.width() as f32,
            height: skin.background.height,
            resizable: true,
            ..Default::default()
        })?;

        Ok(Self {
            controller,
            skin,
            client: None,
            connected: false,
            attached: false,
            events: ButtonState::default(),
        })
    }

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        match SyncClient::connect() {
            Ok(mut s) => {
                let msg = format!("Connected to {}", &s.app_version()?);
                println!("{}", msg);
                s.set_name(String::from(APP_NAME))?;
                self.client = Some(s);
                self.connected = true;
            }
            Err(_) => {
                println!("Not connected to a usb2snes client");
                sleep(time::Duration::from_secs(1));
            }
        };

        Ok(())
    }

    fn attach(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut c) = self.client {
            match c.list_device() {
                Ok(l) => {
                    if !l.is_empty() {
                        c.attach(&l[0])?;
                        let msg = format!("Attached to {}", &l[0]);
                        println!("{}", msg);
                        self.attached = true;
                    }
                }
                Err(_) => println!("Error listing devices"),
            }
        }
        Ok(())
    }
}

impl event::EventHandler for InputViewer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        // const DESIRED_FPS: u32 = 60;
        if !self.connected {
            let _ = self.connect();
        }
        if !self.attached {
            let _ = self.attach();
        }
        if let Some(ref mut c) = self.client {
            if let Ok(e) = self.controller.pushed(c) {
                self.events = e;
            } else {
                self.events = ButtonState::default();
                self.attached = false;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
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
}

fn main() -> Result<GameResult, Box<dyn Error>> {
    /* Setup Configs */
    let app_config = AppConfig::new()?;

    let (mut ctx, event_loop) = ContextBuilder::new(APP_NAME, "ChTBoner")
        .add_resource_path(&app_config.skin.skins_path)
        .window_setup(conf::WindowSetup::default().title(APP_NAME))
        .build()
        .expect("aieee, could not create ggez context!");

    let input_viewer = InputViewer::new(&mut ctx, app_config)?;
    event::run(ctx, event_loop, input_viewer);
}
