use eframe::egui;
use crate::gui::models::Renderer3DApp;

mod gui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native("3d renderer", options, Box::new(|_ctx| Ok(Box::new(Renderer3DApp::default()))))
}
