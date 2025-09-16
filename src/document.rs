use glam::{vec2, Vec2};

pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
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

    pub fn visit<T>( &self, visitor :T)
    where T: Fn(&Line){
        for line in self.lines.iter() {
            visitor(line);
        }
    }
}