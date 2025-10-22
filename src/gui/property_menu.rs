use crate::gui::models::{RenderEngine, Renderer3DApp};
use crate::gui::objects::ObjectType;
use eframe::egui;

pub fn create_property_menu(app: &mut Renderer3DApp, ctx: &egui::Context) {
    egui::SidePanel::right("properties")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("🔧 Properties");
            ui.separator();

            if let Some(selected_idx) = app.selected_object {
                if selected_idx < app.scene_objects.len() {
                    let object = &app.scene_objects[selected_idx];
                    ui.label(format!("Selected: {:?}", object.object_type));
                    ui.separator();

                    // Camera properties
                    if let ObjectType::Camera = object.object_type {
                        ui.collapsing("📷 Camera", |ui| {
                            ui.label("Field of View:");
                            ui.add(
                                egui::Slider::new(&mut app.camera_fov, 1.0..=180.0).suffix("°"),
                            );

                            ui.label("Clip Start:");
                            ui.add(
                                egui::DragValue::new(&mut app.camera_clip_start).speed(0.01),
                            );

                            ui.label("Clip End:");
                            ui.add(egui::DragValue::new(&mut app.camera_clip_end).speed(1.0));
                        });
                    }
                }
            } else {
                ui.label("No object selected");
            }

            ui.separator();

            // Render settings
            ui.collapsing("🎬 Render Settings", |ui| {
                ui.label("Render Engine:");
                egui::ComboBox::from_id_salt("render_engine")
                    .selected_text(format!("{:?}", app.render_engine))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.render_engine, RenderEngine::Eevee, "Eevee");
                        ui.selectable_value(&mut app.render_engine, RenderEngine::Cycles, "Cycles");
                        ui.selectable_value(
                            &mut app.render_engine,
                            RenderEngine::Workbench,
                            "Workbench",
                        );
                    });

                ui.label("Samples:");
                ui.add(egui::Slider::new(&mut app.samples, 1..=1024));

                ui.label("Resolution:");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut app.resolution_x));
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut app.resolution_y));
                });
            });
        });
}
