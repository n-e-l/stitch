use egui::{Color32, Painter, Rect, Sense, Stroke, Ui, Vec2};
use crate::document::{Document, DocumentVisitor, Line};

pub struct DocumentRenderer<'a> {
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

