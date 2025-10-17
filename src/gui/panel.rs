use crate::gui::models::Renderer3DApp;
use eframe::egui;
use eframe::egui::UiBuilder;

pub fn create_panel(app: &mut Renderer3DApp, ctx: &egui::Context) {
    let _ = egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("🎮 3D Viewport");
        
        let available_rect = ui.available_rect_before_wrap();
        ui.scope_builder(UiBuilder::new().max_rect(available_rect), |ui| {
            // TODO
            // Integrate 3D rendering
            let painter = ui.painter();

            // Draw background
            painter.rect_filled(
                available_rect,
                egui::CornerRadius::ZERO,
                egui::Color32::from_gray(45),
            );

            if let Some(selected_idx) = app.selected_object
                && selected_idx < app.scene_objects.len()
            {
                let center = available_rect.center();

                match app.scene_objects[selected_idx].as_str() {
                    "Cube" => {
                        let size = 60.0;
                        let rect = egui::Rect::from_center_size(center, egui::vec2(size, size));
                        painter.rect_stroke(
                            rect,
                            egui::CornerRadius::ZERO,
                            egui::Stroke::new(2.0, egui::Color32::YELLOW),
                            egui::StrokeKind::Outside,
                        );
                    }
                    "Sphere" => {
                        painter.circle_stroke(
                            center,
                            30.0,
                            egui::Stroke::new(2.0, egui::Color32::YELLOW),
                        );
                    }
                    "Cylinder" => {
                        let rect = egui::Rect::from_center_size(center, egui::vec2(40.0, 80.0));
                        painter.rect_stroke(
                            rect,
                            egui::CornerRadius::same(20),
                            egui::Stroke::new(2.0, egui::Color32::YELLOW),
                            egui::StrokeKind::Outside,
                        );
                    }
                    _ => {}
                }
            }
        });
    });
}
