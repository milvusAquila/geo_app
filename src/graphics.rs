use eframe::{self, App, Frame};
use egui::*;
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
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.add(Button::new("New").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_n))).on_hover_text("Create a new file").clicked()
                        || ctx.input_mut(|ui| ui.consume_shortcut(&k.ctrl_n)) {
                        new();
                    }
                    if ui.add(Button::new("Open").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_o))).on_hover_text("Open a file").clicked() {
                        open();
                    }
                    if ui.add(Button::new("Save").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_s))).clicked() {
                        save();
                    }
                    if ui.add(Button::new("Quit").shortcut_text(Context::format_shortcut(&ctx, &k.ctrl_q))).clicked() {
                        close_app(ctx, frame);
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
pub fn new() {
    println!("New");
}
pub fn open() {
    println!("Open");
}
pub fn save() {
    println!("Save");
}
pub fn close_app(_ctx: &Context, frame: &mut Frame) {
    frame.close()
}
