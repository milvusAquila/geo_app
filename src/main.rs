// git add .
// git commit -m "message"
// git push -u origin main

// mod define_frames;
// use crate::define_frames::*;
// commentaire

use std::borrow::Cow;

use eframe::{self, App, Frame};
use egui::{self, ScrollArea, Vec2, FontDefinitions, Context, FontData, FontFamily};

struct CardData { title: String, desc: String, url: String }
struct Headlines { articles: Vec<CardData> }
impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|a: i32| CardData {
            title: format!("title{a}"),
            desc: format!("description{a}"),
            url: format!("https://examples.com/{a}")
        });
        Headlines {
            articles: Vec::from_iter(iter)
        }
     }
   fn configure_font(&self, ctx: Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert("LiberationSerif".to_owned(), FontData::from_static(include_bytes!("../fonts/LiberationSerif.ttf")));
        font_def.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "LiberationSerif".to_owned());
        ctx.set_fonts(font_def);
        
    }
}
impl App for Headlines {
    fn update (&mut self, ctx: &Context, _frame: &mut Frame) {
        let mut central_panel = eframe::egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ScrollArea::new([false, true]).show(ui, |ui: &mut egui::Ui| {
                for a in &self.articles {
                    ui.label(&a.title);
                    ui.label(&a.desc);
                    ui.label(&a.url);
                }
            })
        });
    }
}

fn main() {
    let app: Headlines = Headlines::new();
    let mut win_option = eframe::NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    eframe::run_native("Geo-App", win_option, Box::new(|_| Box::new(app)));
}
