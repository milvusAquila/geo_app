// git add .    git commit -m "message"    git push -u origin main
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe;
use egui::{self, Vec2};
mod graphics;
// mod define_frames;

fn main() -> Result<(), eframe::Error> {
    let mut win_option = eframe::NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(1200., 800.));
    eframe::run_native("Geo_app", win_option, Box::new(|_| Box::new(graphics::Root::default())))
}

/* fn custom_window_frame(ctx: &egui::Context, frame: &mut eframe::Frame, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;
    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };
    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();
        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, frame, title_bar_rect, title);
        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}
fn title_bar_ui(ui: &mut egui::Ui, frame: &mut eframe::Frame, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;
    let painter = ui.painter();
    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );
    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );
    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
        });
    });
} */

/* 
❌
🗗  .set_maximized(false / true)
🗕  .set_minimized(true)
 */
