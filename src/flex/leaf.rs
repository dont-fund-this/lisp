use crate::flex::r#type::{Axis, Node, Rect};

#[allow(dead_code)]
pub fn leaf(grow: f32, fix: Option<f32>) -> Node {
    Node {
        axis: Axis::Col,
        grow,
        fix,
        pad: 0.0,
        kids: Vec::new(),
        rect: Rect::new(0.0, 0.0, 0.0, 0.0),
    }
}
