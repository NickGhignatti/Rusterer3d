use eframe::egui;
use crate::gui::models::Renderer3DApp;

pub fn create_menu(app: &mut Renderer3DApp, ctx: &egui::Context){
    let _ = egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::containers::menu::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    // Reset to default scene
                }
                if ui.button("Open...").clicked() {
                    // Open file dialog
                }
                if ui.button("Save").clicked() {
                    // Save current scene
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    // Undo last action
                }
                if ui.button("Redo").clicked() {
                    // Redo last action
                }
                ui.separator();
                if ui.button("Delete").clicked() {
                    // Delete selected object
                    if let Some(selected) = app.selected_object
                        && selected < app.scene_objects.len()
                        && selected > 2
                    {
                        // Don't allow deleting camera or light
                        app.scene_objects.remove(selected);
                        app.selected_object = None;
                    }
                }
            });

            ui.menu_button("Add", |ui| {
                if ui.button("Mesh > Cube").clicked() {
                    app.scene_objects.push("Cube".to_owned());
                }
                if ui.button("Mesh > Sphere").clicked() {
                    app.scene_objects.push("Sphere".to_owned());
                }
                if ui.button("Mesh > Cylinder").clicked() {
                    app.scene_objects.push("Cylinder".to_owned());
                }
                if ui.button("Light > Point Light").clicked() {
                    app.scene_objects.push("Point Light".to_owned());
                }
            });
        });
    });
}