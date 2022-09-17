use std::sync::{Arc, RwLock, TryLockResult};
use std::thread;
use egui::{Align, Button, Color32, Layout, Vec2};
use egui_extras::RetainedImage;
use crate::server::run_server;
use crate::shared::{AppError, AppStatus, DEFAULT_HOST, DEFAULT_PORT, SharedState};


pub struct App {
    // The logo image
    logo: RetainedImage,
    host: String,
    port: String,
    state: Arc<RwLock<SharedState>>,
}

impl App {
    pub fn new() -> Self {
        let logo = {
            let logo_bytes = include_bytes!("logo.png");
            RetainedImage::from_image_bytes("Logo", logo_bytes)
                .expect("Failed to load logo image")
        };

        let shared_state = SharedState::default();
        let shared_state = Arc::new(RwLock::new(shared_state));

        let app = Self {
            logo,
            host: String::from(DEFAULT_HOST),
            port: format!("{}", DEFAULT_PORT),
            state: shared_state.clone(),
        };

        thread::spawn(move || {
            let shared_state = shared_state.clone();
            run_server(shared_state.clone())
                .expect("Failed to start server");
        });

        app
    }

    pub fn update_state(&mut self) -> Result<(), AppError> {
        let host = self.host.clone();
        if host.is_empty() {
            return Err(AppError::MissingAddress);
        }
        let port = self.port.clone();
        if port.is_empty() {
            return Err(AppError::MissingPort);
        }
        let port = port.parse::<u16>()
            .map_err(|_| AppError::InvalidPort)?;

        let mut state = self.state.write()
            .map_err(|err| AppError::FailedServerStart)?;
        state.host = host.clone();
        state.port = port.clone();
        state.status = AppStatus::Redirecting { host, port };


        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let styles = ui.style_mut();
            styles.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0, 0, 0);

            let width = ui.available_width();
            self.logo.show_max_size(ui, ui.available_size());
            ui.add_space(10f32);
            ui.label("Enter the server address and port in order to connect");
            ui.add_space(10f32);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.label("Address");
                    ui.add_sized(Vec2::new(ui.available_width() / 2.0, 20f32), egui::TextEdit::singleline(&mut self.host));
                });


                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.label("Port");
                    ui.add_sized(Vec2::new(ui.available_width(), 20f32), egui::TextEdit::singleline(&mut self.port));
                });
            });

            ui.add_space(10f32);

            let button_response = ui.add_sized(Vec2::new(width, 30f32), Button::new("Set"));

            if button_response.clicked() {
                match self.update_state() {
                    Ok(_) => {}
                    Err(err) => {
                        if let Ok(mut state) = self.state.write() {
                            state.status = AppStatus::Error(err);
                        }
                    }
                }
            }

            ui.add_space(10f32);
            if let Ok(state) = self.state.read() {
                ui.label(format!("{}", state.status));
            }
        });
    }
}