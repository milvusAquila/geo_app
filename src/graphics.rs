use eframe::{self, App, Frame};
use egui::{self, ScrollArea, FontFamily, Color32, Label, Layout, Hyperlink, Separator, RichText, FontId, TopBottomPanel, Button, TextBuffer};

const PADDING: f32 = 5.0;
pub struct CardData { title: String, desc: String, url: String }
pub struct Headlines { articles: Vec<CardData> }
impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a: i32| CardData {
            title: format!("title{a}"),
            desc: format!("description{a}"),
            url: format!("https://examples.com/{a}")
        });
        Headlines { articles: Vec::from_iter(iter) }
    }
    pub fn render_new_cards(&self, ui: &mut eframe::egui::Ui) {
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
            // ui.begin(egui::containers::Frame::none());
        }
    }
    pub fn render_top_panel(&self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add(Label::new("texte"))
                })
            });
            ui.with_layout(Layout::right_to_left(egui::Align::RIGHT), |ui| {
                let _close_btn = ui.add(Button::new("close"));
                let _refresh_btn = ui.add(Button::new("refresh"));
                let _theme_btn = ui.add(Button::new("change theme"));
            });
        });
    }
}
impl App for Headlines {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.render_top_panel(ctx);
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::new([false, true]).show(ui, |ui| {
                self.render_new_cards(ui)
            })
        });
    }
}


pub struct Root;
impl Root {
    pub fn new() -> Root {
        Root
    }
}
impl App for Root {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.add(Button::new("Quit")).clicked() {
                    frame.close()
                }
            });
        });
    }
}
