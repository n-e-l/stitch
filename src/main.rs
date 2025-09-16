mod editor;
mod document;
mod renderer;

use std::sync::{Arc, Mutex};
use cen::app::App;
use cen::app::app::AppConfig;
use cen::app::gui::{GuiComponent, GuiSystem};
use cen::graphics::Renderer;
use cen::graphics::renderer::{RenderComponent, RenderContext};
use dotenv::dotenv;
use crate::editor::Editor;

struct Application {
    editor: Editor
}

impl Application {

    fn new() -> Application {
        Self {
            editor: Editor::new()
        }
    }
}

impl GuiComponent for Application {
    fn initialize_gui(&mut self, gui: &mut GuiSystem) {
        self.editor.initialize_gui(gui);
    }
    fn gui(&mut self, gui: &GuiSystem, context: &egui_dock::egui::Context) {
        self.editor.gui(gui, context);
    }
}

impl RenderComponent for Application {
    fn initialize(&mut self, renderer: &mut Renderer) {
        self.editor.initialize(renderer);
    }

    fn render(&mut self, ctx: &mut RenderContext) {
        self.editor.render(ctx);
    }
}

fn main() {
    // Initialize .env environment variables
    dotenv().ok();

    let application = Arc::new(Mutex::new(Application::new()));
    App::run(
        AppConfig::default()
            .width(1180)
            .height(1180)
            .log_fps(true)
            .resizable(true)
            .vsync(true),
        application.clone(),
        Some(application.clone())
    );
}
