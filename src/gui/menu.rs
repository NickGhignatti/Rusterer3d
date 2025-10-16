use eframe::egui;
use crate::gui::models::{Renderer3DApp, Tool, ViewportMode};

pub fn create_menu(app: &mut Renderer3DApp, ctx: &egui::Context) {
    let _ = egui::TopBottomPanel::bottom("menu").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Tools:");
            ui.separator();

            if ui.selectable_label(app.current_tool == Tool::Select, "🔍 Select").clicked() {
                app.current_tool = Tool::Select;
            }
            if ui.selectable_label(app.current_tool == Tool::Move, "↔ Move").clicked() {
                app.current_tool = Tool::Move;
            }
            if ui.selectable_label(app.current_tool == Tool::Rotate, "🔄 Rotate").clicked() {
                app.current_tool = Tool::Rotate;
            }
            if ui.selectable_label(app.current_tool == Tool::Scale, "📏 Scale").clicked() {
                app.current_tool = Tool::Scale;
            }
            if ui.selectable_label(app.current_tool == Tool::Extrude, "📤 Extrude").clicked() {
                app.current_tool = Tool::Extrude;
            }

            ui.separator();
            ui.label("Viewport:");
            egui::ComboBox::from_id_salt("viewport_mode")
                .selected_text(format!("{:?}", app.viewport_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.viewport_mode, ViewportMode::Solid, "Solid");
                    ui.selectable_value(&mut app.viewport_mode, ViewportMode::Wireframe, "Wireframe");
                    ui.selectable_value(&mut app.viewport_mode, ViewportMode::Material, "Material");
                    ui.selectable_value(&mut app.viewport_mode, ViewportMode::Rendered, "Rendered");
                });
        });
    });
}