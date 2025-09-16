use std::collections::HashMap;
use ash::vk::{DescriptorSetLayoutBinding, DescriptorType, PushConstantRange, ShaderStageFlags};
use bytemuck::{Pod, Zeroable};
use cen::app::gui::{GuiComponent, GuiSystem};
use cen::graphics::Renderer;
use cen::graphics::renderer::{RenderComponent, RenderContext};
use cen::vulkan::{ComputePipeline, DescriptorSetLayout};
use egui::{Color32, Painter, Rect, Scene, Sense, Stroke, Ui, Vec2};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use crate::document::{Document, DocumentVisitor, Line};

pub struct Editor {
    pub tree: DockState<String>,
    tab_viewer: Option<TabViewer>,
    pipeline: Option<ComputePipeline>,
    layout: Option<DescriptorSetLayout>
}

impl Editor {
    pub(crate) fn new() -> Self {

        let mut tree = DockState::new(vec!["view".to_owned()]);

        let [_, _] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.3, vec!["tools".to_owned()]);

        Self {
            tree,
            pipeline: None,
            tab_viewer: None,
            layout: None
        }
    }
}

struct DocumentRenderer<'a> {
    painter: Painter,
    document: &'a Document
}

impl<'a> DocumentRenderer<'a> {
    pub fn new(ui: &'a mut Ui, document: &'a Document) -> Self {
        let size = Vec2::new(document.size().x, document.size().y);
        let (_, painter) = ui.allocate_painter(size, Sense::empty());
        Self {
            painter,
            document
        }
    }

    pub fn clip_rect(&self) -> Rect {
        self.painter.clip_rect()
    }

    pub fn render(&self) {
        self.document.visit(self);
    }
}

impl DocumentVisitor for DocumentRenderer<'_> {
    fn visit(&self, line: &Line) {
        let start = egui::Pos2::new(line.a.x, line.a.y);
        let end = egui::Pos2::new(line.b.x, line.b.y);
        let stroke = Stroke::new(2.0, Color32::BLUE);
        self.painter.line_segment([start, end], stroke);
    }
}

struct TabViewer {
    scene_rect: Rect,
    scene_pointer: Vec2,
    document: Document
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if tab.as_str() == "view" {

            ui.input(|input| {
                if let Some(p) = input.pointer.hover_pos() {
                    // Read where we are on the image
                    let frame_rect = ui.min_rect();
                    let mouse_frame_pos = p - frame_rect.min;
                    self.scene_pointer = mouse_frame_pos / frame_rect.size() * self.scene_rect.size() + self.scene_rect.min.to_vec2();
                }
            });

            egui::Frame::group(ui.style())
                .inner_margin(0.0)
                .show(ui, |ui| {
                    let scene = Scene::new()
                        .max_inner_size([350.0, 1000.0])
                        .zoom_range(0.1..=30.0);

                    let mut inner_rect = Rect::NAN;
                    let response = scene
                        .show(ui, &mut self.scene_rect, |ui| {
                            
                            // Graphics contents
                            let renderer = DocumentRenderer::new(ui, &self.document);
                            renderer.render();

                            inner_rect = renderer.clip_rect();
                        })
                        .response;

                    if response.double_clicked() {
                        self.scene_rect = inner_rect;
                    }
                });
        }
    }
}


impl GuiComponent for Editor {
    fn initialize_gui(&mut self, _: &mut GuiSystem) {
        self.tab_viewer = Some(TabViewer {
            document: Document::new(),
            scene_rect: Rect::ZERO,
            scene_pointer: Default::default(),
        });
    }

    fn gui(&mut self, _: &GuiSystem, context: &egui::Context) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(context.style().as_ref()))
            .show(context, self.tab_viewer.as_mut().unwrap());
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct PushConstants {
    cursor: Vec2
}

impl RenderComponent for Editor {
    fn initialize(&mut self, renderer: &mut Renderer) {

        // Initialize shader
        let bindings = [
            DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_IMAGE)
                .stage_flags(ShaderStageFlags::COMPUTE)
        ];

        let layout = DescriptorSetLayout::new_push_descriptor(
            &renderer.device,
            &bindings
        );

        let push_constants = PushConstantRange::default()
            .size(size_of::<PushConstants>() as u32)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .offset(0);

        let macros: HashMap<String, String> = HashMap::new();
        self.pipeline = Some(ComputePipeline::new(
            &renderer.device,
            "shaders/brush.comp".parse().unwrap(),
            &[layout.clone()],
            &[push_constants],
            &macros
        ).unwrap());
        self.layout = Some(layout);

    }

    fn render(&mut self, _: &mut RenderContext) {
    }
}