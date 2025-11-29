use crate::controller::button_state::ButtonState;
use crate::controller::controller_impl::ControllerData;

use crate::configuration::AppConfig;
use crate::skins::skin::Skin;
use ggez::{
    conf, event,
    graphics::{self, Color, DrawParam, Text, TextFragment},
    Context, GameResult,
};
use rusb2snes::{SyncClient, USB2SnesEndpoint};
use std::error::Error;

pub const APP_NAME: &str = "Snes Input Display";

pub struct InputViewer {
    controller: ControllerData,
    skin: Skin,
    client: Option<SyncClient>,
    events: ButtonState,
    message: Option<String>,
    prev_message: Option<String>,
    endpoint: USB2SnesEndpoint,
}

impl InputViewer {
    pub fn new(ctx: &mut Context, config: AppConfig) -> Result<Self, Box<dyn Error>> {
        let controller = ControllerData::new(&config.controller)?;

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

        let endpoint = config.usb2snes.unwrap_or_default();

        Ok(Self {
            controller,
            skin,
            client: None,
            events: ButtonState::default(),
            message: None,
            prev_message: None,
            endpoint,
        })
    }

    fn connect(&mut self) -> Result<Option<SyncClient>, Box<dyn Error>> {
        let client = match SyncClient::connect(&self.endpoint) {
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
                                Some("Not attached to usb2snes compatible endpoint".to_string());
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
            Some(ref mut c) => match self.controller.current_addresses.pushed(c) {
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
        if self.message != self.prev_message {
            let deb = match &self.message {
                Some(s) => s,
                None => "",
            };
            println!("{}", deb);
            self.prev_message = self.message.clone();
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
        if let Some(ref msg) = self.message {
            let text = Text::new(TextFragment {
                text: msg.to_string(),
                color: Some(Color::RED),
                ..Default::default()
            });
            canvas.draw(&text, DrawParam::default());
        }
        canvas.finish(ctx)
    }
}
