// mod define_frames;
// use crate::define_frames::*;

use eframe;
use eframe::{App, Frame};
use egui::{self, ScrollArea};

struct CardData { title: String, desc: String, url: String }

struct Headlines {
    articles: Vec<CardData>
}
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
}
impl App for Headlines {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ScrollArea::auto_sized().show(ui, |ui: &mut egui::Ui| {
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
    let win_option = eframe::NativeOptions::default();
    eframe::run_native("Geo-App", win_option, Box::new(|_| Box::new(app)));
}
