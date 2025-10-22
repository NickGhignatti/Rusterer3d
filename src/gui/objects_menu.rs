use eframe::egui;
use crate::gui::models::Renderer3DApp;

pub fn create_objects_menu(app: &mut Renderer3DApp, ctx: &egui::Context) {
    let _ = egui::SidePanel::left("outliner").resizable(true).show(ctx, |ui| {
        ui.heading("📂 Scene Outliner");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, object) in app.scene_objects.iter().enumerate() {
                let is_selected = app.selected_object == Some(i);
                if ui.selectable_label(is_selected, format!("🔗 {:?}", object.object_type)).clicked() {
                    app.selected_object = Some(i);
                }
            }
        });

        ui.separator();
        ui.label("Scene Statistics:");
        ui.label(format!("Objects: {}", app.scene_objects.len()));
    });
}