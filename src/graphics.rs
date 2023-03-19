use eframe::{self, App, Frame};
use egui::*;

#[derive(Clone, Copy)]
pub struct Root  {
    ctrl_n: KeyboardShortcut,
    ctrl_o: KeyboardShortcut,
    ctrl_s: KeyboardShortcut,
    ctrl_q: KeyboardShortcut,
}
impl Root {
    pub fn keyboard(self, ctx: &Context, frame: &mut Frame) {
        if ctx.input_mut(|ui| ui.consume_shortcut(&self.ctrl_n)) {
            new();
        }
        if ctx.input_mut(|ui| ui.consume_shortcut(&self.ctrl_o)) {
            open();
        }
        if ctx.input_mut(|ui| ui.consume_shortcut(&self.ctrl_s)) {
            save();
        }
        if ctx.input_mut(|ui| ui.consume_shortcut(&self.ctrl_q)) {
            close_app(ctx, frame);
        }
    }
    pub fn menu_file(self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        if ui.add(Button::new("New").shortcut_text(Context::format_shortcut(&ctx, &self.ctrl_o))).on_hover_text("Create a new file").clicked() {
            new();
        }
        if ui.add(Button::new("Open").shortcut_text(Context::format_shortcut(&ctx, &self.ctrl_o))).on_hover_text("Open a file").clicked() {
            open();
        }
        if ui.add(Button::new("Save").shortcut_text(Context::format_shortcut(&ctx, &self.ctrl_s))).clicked() {
            save();
        }
        if ui.add(Button::new("Quit").shortcut_text(Context::format_shortcut(&ctx, &self.ctrl_q))).clicked() {
            close_app(ctx, frame);
        }
    }
    pub fn menu_shape(self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {

    }
}
impl Default for Root {
    fn default() -> Self {
        Root {
            ctrl_n: KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::N},
            ctrl_o: KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::O},
            ctrl_s: KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::S},
            ctrl_q: KeyboardShortcut {modifiers: Modifiers::CTRL, key: Key::Q},          
        }
    }
}
impl App for Root {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.keyboard(ctx, frame);
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    self.menu_file(ctx, frame, ui);
                });
                egui::menu::menu_button(ui, "Edit", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui)
                });
                egui::menu::menu_button(ui, "Shape", |ui| {
                    if ui.add(Button::new("Circle")).clicked() {
                        self.menu_shape(ctx, frame, ui)
                    }
                })
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                let grid = ui.painter();
                grid.circle(Pos2::new(50., 60.), 10., Color32::RED, Stroke::default())
            });
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
