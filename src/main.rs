#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controller;
mod skins;
use controller::{button_state::ButtonState, controller_impl::Controller};

use ggez::{
    conf, event,
    graphics::{self, Color, DrawParam, Text, TextFragment},
    Context, ContextBuilder, GameResult,
};
use rusb2snes::SyncClient;
use skins::skin::Skin;
use std::{env, error::Error};

use configuration::AppConfig;

const APP_NAME: &str = "Snes Input Display";

struct InputViewer {
    controller: Controller,
    skin: Skin,
    client: Option<SyncClient>,
    events: ButtonState,
    message: Option<String>,
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
            events: ButtonState::default(),
            message: None,
        })
    }

    fn connect(&mut self) -> Result<Option<SyncClient>, Box<dyn Error>> {
        let client = match SyncClient::connect() {
            Ok(mut s) => {
                s.set_name(String::from(APP_NAME))?;
                match s.list_device() {
                    Ok(l) => {
                        if !l.is_empty() {
                            s.attach(&l[0])?;
                            let msg = format!("Attached to {}", &l[0]);
                            println!("{}", msg);
                        } else {
                            self.message =
                                Some("Not attached to usb2snes comptable endpoint".to_string());
                        }
                    }
                    Err(_) => {
                        println!("No device available");
                        return Ok(Some(s));
                    }
                }
                Some(s)
            }
            Err(_) => {
                self.message = Some("Not connected to usb2snes websocket".to_string());
                None
            }
        };

        Ok(client)
    }
}

impl event::EventHandler for InputViewer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        match self.client {
            Some(ref mut c) => match self.controller.pushed(c) {
                Ok(e) => {
                    self.events = e;
                    self.message = None;
                }
                Err(_) => {
                    self.events = ButtonState::default();
                    self.client = None;
                }
            },
            None => match self.connect() {
                Ok(c) => self.client = c,
                Err(_) => self.client = None,
            },
        };

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
        if let Some(ref msg) = self.message {
            let text = Text::new(TextFragment {
                text: msg.to_string(),
                color: Some(Color::new(1.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            });
            canvas.draw(&text, DrawParam::default());
        }
        canvas.finish(ctx)
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
