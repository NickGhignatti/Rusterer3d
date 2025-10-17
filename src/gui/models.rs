#![allow(dead_code, unused)]

use eframe::Frame;
use eframe::egui::Context;
use crate::gui::menu::create_menu;
use crate::gui::panel::create_panel;
use crate::gui::toolbar::create_toolbar;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum RenderEngine {
    Eevee,
    Cycles,
    Workbench,
}

#[derive(Clone)]
pub struct Renderer3DApp {
    // Scene data
    pub(crate) scene_objects: Vec<String>,
    pub(crate) selected_object: Option<usize>,

    // Tool
    pub(crate) current_tool: Tool,

    // Viewport settings
    pub(crate) viewport_mode: ViewportMode,

    // Camera settings
    camera_fov: f32,
    camera_clip_start: f32,
    camera_clip_end: f32,

    // Render settings
    render_engine: RenderEngine,
    samples: i32,
    resolution_x: i32,
    resolution_y: i32,
}

impl Default for Renderer3DApp {
    fn default() -> Self {
        Self {
            scene_objects: vec!["Camera".to_owned(), "Light".to_owned()],
            selected_object: None,
            current_tool: Tool::Select,
            viewport_mode: ViewportMode::Solid,
            camera_fov: 50.0,
            camera_clip_start: 0.1,
            camera_clip_end: 1_000.0,
            render_engine: RenderEngine::Eevee,
            samples: 64,
            resolution_x: 1_920,
            resolution_y: 1_080,
        }
    }
}

impl eframe::App for Renderer3DApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Menu bar
        create_menu(self, ctx);
        // Toolbar
        create_toolbar(self, ctx);
        // Central panel ~ 3d viewport
        create_panel(self, ctx);
    }
}