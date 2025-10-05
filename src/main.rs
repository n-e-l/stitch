mod editor;
mod document;
mod renderer;

use std::sync::{Arc, Mutex};
use cen::app::Cen;
use cen::app::app::AppConfig;
use cen::app::component::{Component, ComponentRegistry};
use cen::app::engine::InitContext;
use cen::app::gui::{GuiComponent, GuiSystem};
use cen::graphics::Renderer;
use cen::graphics::renderer::{RenderComponent, RenderContext};
use dotenv::dotenv;
use crate::editor::Editor;

struct Application {
    editor: Editor
}

impl Application {

    fn new(ctx: &mut InitContext) -> Application {
        Self {
            editor: Editor::new(ctx)
        }
    }
}

impl GuiComponent for Application {
    fn gui(&mut self, gui: &GuiSystem, context: &egui_dock::egui::Context) {
        self.editor.gui(gui, context);
    }
}

impl RenderComponent for Application {
    fn render(&mut self, ctx: &mut RenderContext) {
        self.editor.render(ctx);
    }
}

fn main() {
    // Initialize .env environment variables
    dotenv().ok();

    Cen::run(
        AppConfig::default()
            .width(1180)
            .height(1180)
            .log_fps(true)
            .resizable(true)
            .vsync(true),
        Box::new(|ctx| {
            let application = Arc::new(Mutex::new(Application::new(ctx)));
            ComponentRegistry::new()
                .register(Component::Render(application.clone()))
                .register(Component::Gui(application.clone()))
        })
    );
}
