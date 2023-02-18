// git add .
// git commit -m "message"
// git push -u origin main

// mod define_frames;
// use crate::define_frames::*;
mod graphics;
use eframe;
use egui::{self, Vec2};

fn main() {
    let app: graphics::Headlines = graphics::Headlines::new();
    let mut win_option = eframe::NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    let _ = eframe::run_native("Geo-App", win_option, Box::new(|_| Box::new(app)));
}
