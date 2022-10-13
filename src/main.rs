#![windows_subsystem = "windows"]
use egui::Vec2;
use crate::app::App;

mod server;
mod app;
mod shared;
mod hosts;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.resizable = false;
    native_options.initial_window_size = Some(Vec2::new(300f32, 290f32));
    eframe::run_native("Pocket Relay Client", native_options,
                       Box::new(|_| Box::new(App::new())));
}
