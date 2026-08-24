#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[allow(dead_code)]
impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Row,
    Col,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Node {
    pub axis: Axis,
    pub grow: f32,
    pub fix: Option<f32>,
    pub pad: f32,
    pub kids: Vec<Node>,
    pub rect: Rect,
}

#[allow(dead_code)]
impl Node {
    pub fn new(axis: Axis, pad: f32) -> Self {
        Self {
            axis,
            grow: 1.0,
            fix: None,
            pad,
            kids: Vec::new(),
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}
