use std::f32::consts::PI;
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

fn gen_pos(t: f32) -> Vec2 {
    let a = t * PI * 2.;
    vec2( f32::cos(a), f32::sin(a), ) * 100. + vec2(200., 200.)
}

impl Document {
    pub fn new() -> Document {

        let mut lines = vec![
        ];

        let mut last_pos = gen_pos(0.);
        for i in 1..100 {
            let t = i as f32 / 99.0;
            let pos = gen_pos(t);
            lines.push( Line { a: pos, b: last_pos } );
            last_pos = pos;
        }


        // let lines = vec![
        //     Line { a: vec2(0.0, 100.0), b: vec2(100.0, 100.0) },
        //     Line { a: vec2(100.0, 100.0), b: vec2(100.0, 0.0) },
        //     Line { a: vec2(100.0, 0.0), b: vec2(0.0, 0.0) },
        //     Line { a: vec2(0.0, 0.0), b: vec2(0.0, 100.0) },
        // ];

        Document {
            lines
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