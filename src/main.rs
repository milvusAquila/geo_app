// git add .
// git commit -m "message"
// git push -u origin main

// mod define_frames;
// use crate::define_frames::*;
use eframe::{self, App, Frame};
use egui::{self, ScrollArea, Vec2, FontFamily, Color32, Label, Layout, Hyperlink, Separator, RichText, FontId};

struct CardData { title: String, desc: String, url: String }
struct Headlines { articles: Vec<CardData> }
impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|a: i32| CardData {
            title: format!("title{a}"),
            desc: format!("description{a}"),
            url: format!("https://examples.com/{a}")
        });
        Headlines { articles: Vec::from_iter(iter) }
    }
    fn render_new_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            let title = RichText::new(format!("Play {}", a.title))
                .font(FontId { size: 32.0, family: FontFamily::Proportional })
                .color(Color32::GREEN);
            ui.label(title);
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc);
            ui.add(desc);
            ui.style_mut().visuals.hyperlink_color = Color32::BROWN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more", &a.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}
impl App for Headlines {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ScrollArea::new([false, true]).show(ui, |ui: &mut egui::Ui| {
                self.render_new_cards(ui)
            })
        });
    }
}
const PADDING: f32 = 5.0;
fn main() {
    let app: Headlines = Headlines::new();
    let mut win_option = eframe::NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    eframe::run_native("Geo-App", win_option, Box::new(|_| Box::new(app)));
}
