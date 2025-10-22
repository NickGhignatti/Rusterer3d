#![allow(dead_code, unused)]

use crate::gui::menu::create_menu;
use crate::gui::objects::Object;
use crate::gui::objects::ObjectType::{Camera, Light};
use crate::gui::objects_menu::create_objects_menu;
use crate::gui::panel::create_panel;
use crate::gui::property_menu::create_property_menu;
use crate::gui::toolbar::create_toolbar;
use eframe::{egui, Frame};
use eframe::egui::Context;
use eframe::egui::accesskit::{Point, Size};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewportMode {
    Solid,
    Wireframe,
    Material,
    Rendered,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Select,
    Move,
    Rotate,
    Scale,
    Extrude,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RenderEngine {
    Eevee,
    Cycles,
    Workbench,
}

#[derive(Clone)]
pub struct Renderer3DApp {
    // Scene data
    pub(crate) scene_objects: Vec<Object>,
    pub(crate) selected_object: Option<usize>,

    // Tool
    pub(crate) current_tool: Tool,

    // Lateral menus
    pub(crate) objects_list_menu: bool,
    pub(crate) objects_properties_menu: bool,

    // Viewport settings
    pub(crate) viewport_mode: ViewportMode,

    // Camera settings
    pub(crate) camera_fov: f32,
    pub(crate) camera_clip_start: f32,
    pub(crate) camera_clip_end: f32,

    // Render settings
    pub(crate) render_engine: RenderEngine,
    pub(crate) samples: i32,
    pub(crate) resolution_x: i32,
    pub(crate) resolution_y: i32,

    // Drawing panel
    pub(crate) drawing_panel_center: Point,
}

impl Default for Renderer3DApp {
    fn default() -> Self {
        Self {
            scene_objects: vec![
                Object::new(Camera, Point::ZERO, Size::ZERO),
                Object::new(Light, Point::ZERO, Size::ZERO),
            ],
            selected_object: None,
            current_tool: Tool::Select,
            objects_list_menu: false,
            objects_properties_menu: false,
            viewport_mode: ViewportMode::Solid,
            camera_fov: 50.0,
            camera_clip_start: 0.1,
            camera_clip_end: 1_000.0,
            render_engine: RenderEngine::Eevee,
            samples: 64,
            resolution_x: 1_920,
            resolution_y: 1_080,
            drawing_panel_center: Point::ZERO,
        }
    }
}

impl eframe::App for Renderer3DApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Central panel ~ 3d viewport
        create_panel(self, ctx);
        // Menu bar
        create_menu(self, ctx);
        // Toolbar
        create_toolbar(self, ctx);

        // Lateral menus
        if self.objects_list_menu {
            create_objects_menu(self, ctx);
        }
        if self.objects_properties_menu {
            create_property_menu(self, ctx);
        }
    }
}
