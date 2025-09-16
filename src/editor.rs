use std::collections::HashMap;
use ash::vk;
use ash::vk::{AccessFlags, BufferImageCopy, BufferUsageFlags, DescriptorSet, DescriptorSetLayoutBinding, DescriptorType, DeviceSize, ImageLayout, ImageUsageFlags, ImageView, PipelineStageFlags, PushConstantRange, Sampler, ShaderStageFlags, WriteDescriptorSet};
use bytemuck::{Pod, Zeroable};
use cen::app::gui::{GuiComponent, GuiSystem};
use cen::graphics::Renderer;
use cen::graphics::renderer::{RenderComponent, RenderContext};
use cen::vulkan::{Buffer, CommandBuffer, ComputePipeline, DescriptorSetLayout, Image};
use egui::{vec2, Color32, ImageSize, ImageSource, Pos2, Rect, Scene, Sense, Stroke, TextureId, Vec2, Widget};
use egui::load::SizedTexture;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use gpu_allocator::MemoryLocation;
use image::{EncodableLayout, GenericImageView};

pub struct Editor {
    pub tree: DockState<String>,
    tab_viewer: Option<TabViewer>,
    pipeline: Option<ComputePipeline>,
    layout: Option<DescriptorSetLayout>
}

impl Editor {
    pub(crate) fn new() -> Self {

        let mut tree = DockState::new(vec!["view".to_owned()]);

        let [a, b] =
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

struct TabViewer {
    scene_rect: Rect,
    scene_pointer: Vec2,
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
                            let (response, painter) = ui.allocate_painter(vec2(500.0, 500.0), Sense::empty());
                            let start = Pos2::new(50.0, 100.0);
                            let end = Pos2::new(200.0, 100.0);
                            let stroke = Stroke::new(2.0, Color32::BLUE);
                            painter.line_segment([start, end], stroke);
                            
                            inner_rect = painter.clip_rect();
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
    fn initialize_gui(&mut self, gui: &mut GuiSystem) {
        self.tab_viewer = Some(TabViewer {
            scene_rect: Rect::ZERO,
            scene_pointer: Default::default(),
        });
    }

    fn gui(&mut self, gui: &GuiSystem, context: &egui::Context) {
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

    fn render(&mut self, ctx: &mut RenderContext) {
    }
}