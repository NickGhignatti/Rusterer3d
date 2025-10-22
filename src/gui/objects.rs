use eframe::egui::accesskit::{Point, Size};

#[derive(Clone, Debug)]
pub enum ObjectType {
    Sphere,
    Cube,
    Cylinder,
    Camera,
    Light
}

#[derive(Clone)]
pub struct Object {
    pub(crate) object_type: ObjectType,
    pub(crate) coordinates: Point,
    pub(crate) size: Size
}

impl Default for Object {
    fn default() -> Self {
        Self {
            object_type: ObjectType::Cube,
            coordinates: Point::new(0.0, 0.0),
            size: Size::new(1.0, 1.0)
        }
    }
}

impl Object {
    pub fn new(object_type: ObjectType, coordinates: Point, size: Size) -> Self {
        Self {
            object_type,
            coordinates,
            size
        }
    }
}