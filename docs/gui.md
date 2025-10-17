The GUI is implemented by the usage of [egui](https://github.com/emilk/egui), a simple GUI library for rust.

The `App` concept of egui is mapped to our `Renderer3DApp` which contains all the fields which are necessary.

```rust
struct Renderer3DApp {
    scene_objects: Vec<String>,
    selected_object: Option<usize>,
    pub(crate) current_tool: Tool,
    pub(crate) viewport_mode: ViewportMode,
    ...
}
```
The App is created through the union of different parts like a menu, a toolbar, etc

```rust
impl eframe::App for Renderer3DApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        create_menu(self, ctx);
        create_toolbar();
        create_panel();
        ...
    }
}
```