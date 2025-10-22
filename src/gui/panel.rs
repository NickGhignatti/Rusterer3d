use crate::gui::models::Renderer3DApp;
use crate::gui::objects::{Object, ObjectType};
use eframe::egui;
use eframe::egui::{Pos2, UiBuilder};
use eframe::egui::accesskit::Point;

fn match_visual_element(object: &Object, painter: &egui::Painter) {
    match object.object_type {
        ObjectType::Cube => {
            let size = object.size;
            let position = object.coordinates;
            let rect = egui::Rect::from_center_size(
                Pos2::new(position.x as f32, position.y as f32),
                egui::vec2(size.width as f32, size.height as f32),
            );
            painter.rect_stroke(
                rect,
                egui::CornerRadius::ZERO,
                egui::Stroke::new(2.0, egui::Color32::YELLOW),
                egui::StrokeKind::Outside,
            );
        }
        ObjectType::Sphere => {
            let position = object.coordinates;
            painter.circle_stroke(
                Pos2::new(position.x as f32, position.y as f32),
                object.size.width as f32 / 2.0,
                egui::Stroke::new(2.0, egui::Color32::GREEN),
            );
        }
        ObjectType::Cylinder => {
            let size = object.size;
            let position = object.coordinates;
            let rect = egui::Rect::from_center_size(
                Pos2::new(position.x as f32, position.y as f32),
                egui::vec2(size.width as f32, size.height as f32),
            );
            painter.rect_stroke(
                rect,
                egui::CornerRadius::same(size.width as u8),
                egui::Stroke::new(2.0, egui::Color32::BLUE),
                egui::StrokeKind::Outside,
            );
        }
        _ => {}
    }
}

pub fn create_panel(app: &mut Renderer3DApp, ctx: &egui::Context) {
    let _ = egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("🎮 3D Viewport");

        let available_rect = ui.available_rect_before_wrap();
        ui.scope_builder(UiBuilder::new().max_rect(available_rect), |ui| {
            // This would be where you integrate your 3D rendering
            // For now, we'll just show a placeholder
            let painter = ui.painter();
            let center = available_rect.center();

            app.drawing_panel_center = Point::new(center.x as f64, center.y as f64);

            // Draw background
            painter.rect_filled(
                available_rect,
                egui::CornerRadius::ZERO,
                egui::Color32::from_gray(45),
            );

            for object in app.scene_objects.clone() {
                match_visual_element(&object, painter);
            }

            // Draw simple representation of objects
            if let Some(selected_idx) = app.selected_object
                && selected_idx < app.scene_objects.len()
            {
                match_visual_element(&app.scene_objects[selected_idx], painter);
            }
        });
    });
}
