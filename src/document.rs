use glam::{vec2, Vec2};

pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
}

pub trait DocumentVisitor {
    fn visit(&self, line: &Line);
}

pub struct Document {
    lines: Vec<Line>
}

impl Document {
    pub fn new() -> Document {
        Document {
            lines: vec![
                Line { a: vec2(0.0, 100.0), b: vec2(100.0, 100.0) },
                Line { a: vec2(100.0, 100.0), b: vec2(100.0, 0.0) },
                Line { a: vec2(100.0, 0.0), b: vec2(0.0, 0.0) },
                Line { a: vec2(0.0, 0.0), b: vec2(0.0, 100.0) },
                Line { a: vec2(0.0, 100.0), b: vec2(100.0, 100.0) },
            ]
        }
    }

    pub fn size(&self ) -> Vec2 {
        vec2(500.0, 500.0)
    }

    pub fn visit( &self, visitor: &dyn DocumentVisitor)
    {
        for line in self.lines.iter() {
            visitor.visit(line);
        }
    }
}