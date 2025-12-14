use crate::configuration::AppConfig;
use crate::controller::button_state::ButtonState;
use crate::controller::controller_impl::ControllerData;
use crate::skins::skin::SkinData;

use ggez::{
    conf, event,
    graphics::{self, Color, DrawParam, Text, TextFragment},
    input::keyboard::KeyCode,
    Context, GameResult,
};
use rusb2snes::{SyncClient, USB2SnesEndpoint};
use std::error::Error;
// use winit::keyboard::{Key, NamedKey};

pub const APP_NAME: &str = "Snes Input Display";

pub struct InputViewer {
    controller: ControllerData,
    skin: SkinData,
    client: Option<SyncClient>,
    events: ButtonState,
    error_message: Option<String>,
    prev_error_message: Option<String>,
    window_title: String,
    endpoint: USB2SnesEndpoint,
}

impl InputViewer {
    pub fn new(ctx: &mut Context, config: AppConfig) -> Result<Self, Box<dyn Error>> {
        let controller = ControllerData::new(&config.controller)?;

        let skin = SkinData::new(&config.skin, ctx)?;

        // Set the window size
        ctx.gfx.set_mode(conf::WindowMode {
            width: skin.current_skin.background.image.width() as f32,
            height: skin.current_skin.background.height,
            resizable: true,
            ..Default::default()
        })?;

        let endpoint = config.usb2snes.unwrap_or_default();
        let window_title = format!("{} - {}", APP_NAME, controller.layout_name);
        ctx.gfx.set_window_title(&window_title);

        Ok(Self {
            controller,
            skin,
            client: None,
            events: ButtonState::default(),
            error_message: None,
            prev_error_message: None,
            window_title,
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
                            self.error_message =
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
                self.error_message = Some("Not connected to usb2snes websocket".to_string());
                None
            }
        };

        Ok(client)
    }

    fn update_title(&mut self) {
        self.window_title = format!("{} - {}", APP_NAME, self.controller.layout_name);
    }
}

impl event::EventHandler for InputViewer {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_just_released(KeyCode::J) {
            self.controller.get_next_layout();
            self.window_title = format!("{} - {}", APP_NAME, self.controller.layout_name);
        } else if ctx.keyboard.is_key_just_released(KeyCode::K) {
            self.controller.get_prev_layout();
            self.update_title();
        } else {
            match self.client {
                Some(ref mut c) => match self.controller.current_addresses.pushed(c) {
                    Ok(e) => {
                        self.events = e;
                        self.error_message = None;
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
        }

        if self.error_message != self.prev_error_message {
            let deb = match &self.error_message {
                Some(s) => s,
                None => "",
            };
            println!("{}", deb);
            self.prev_error_message = self.error_message.clone();
        }
        let window_title = format!("{} - {}", APP_NAME, self.controller.layout_name);
        ctx.gfx.set_window_title(&window_title);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        // draw background
        canvas.draw(&self.skin.current_skin.background.image, DrawParam::new());

        // Draw inputs
        self.events.iter().for_each(|event| {
            let button_image = &self.skin.current_skin.buttons[event].image;
            canvas.draw(
                button_image,
                DrawParam::default().dest(self.skin.current_skin.buttons[event].rect.point()),
            );
        });

        // draw error message
        if let Some(ref msg) = self.error_message {
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
