use eframe::{self, App, Frame};
use egui::*;

/* const PADDING: f32 = 5.0;
pub struct CardData { title: String, desc: String, url: String }
pub struct Headlines { articles: Vec<CardData> }
impl Headlines {
    pub fn _new() -> Headlines {
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
 */
pub struct KeyboardSettings {
    ctrl_n : KeyboardShortcut,
    ctrl_o : KeyboardShortcut,
    ctrl_s : KeyboardShortcut,
    ctrl_q : KeyboardShortcut,
}
impl KeyboardSettings {
    pub fn new() -> KeyboardSettings {
        KeyboardSettings {
            ctrl_n : KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::N},
            ctrl_o : KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::O},
            ctrl_s : KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::S},
            ctrl_q : KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::Q},
        }
    }
}

pub struct Root;
impl App for Root {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let k = KeyboardSettings::new();
        if ctx.input_mut(|ui| ui.consume_shortcut(&k.ctrl_n)) {
            println!("Keyboard ctrl + n");
        }
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.add(Button::new("New").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_n))).on_hover_text("Create a new file").clicked() {
                        println!("New");
                    }
                    if ui.add(Button::new("Open").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_o))).on_hover_text("Open a file").clicked() {
                        println!("Open");
                    }
                    if ui.add(Button::new("Save").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_s))).clicked() {
                        println!("Save");
                    }
                    if ui.add(Button::new("Quit").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_q))).clicked() {
                        println!("Quit");
                        close_app(ctx, frame)
                    }
                });
                egui::menu::menu_button(ui, "Edit", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui)
                });
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            });
            if ui.input(|ui| ui.key_pressed(Key::S)) {
                    println!("S");
                }
        });
    }
}
pub fn keyboard_manager(ctx: &Context, frame: &mut Frame) {
}
pub fn close_app(_ctx: &Context, frame: &mut Frame) {
    frame.close()
}
