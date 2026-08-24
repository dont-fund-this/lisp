use ratatui::layout::Rect;
use crate::pane::Mode;

#[derive(Default, Clone)]
pub struct Hits {
    pub run: Rect,
    pub rset: Rect,
    pub help: Rect,
    pub tabs: Vec<(Rect, usize)>,
    pub new: Rect,
    pub tree: Rect,
    pub items: Vec<(Rect, usize)>,
    pub text: Rect,
    pub num_w: usize,
    pub modes: Vec<(Rect, Mode)>,
    pub pane: Rect,
    pub v_bar: Rect,
    pub h_bar: Rect,
}
