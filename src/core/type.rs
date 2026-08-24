use crate::eval::Vm;
use crate::pane::Pane;
use crate::text::Book;
use crate::tree::Nav;
use crate::view::Hits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Tree,
    Text,
    Pane,
}

pub struct App {
    pub vm: Vm,
    pub text: Book,
    pub tree: Nav,
    pub pane: Pane,
    pub focus: Focus,
    pub show_tree: bool,
    pub show_help: bool,
    pub should_quit: bool,

    pub tree_w: u16,
    pub split_pct: u16,
    pub drag_v: bool,
    pub drag_h: bool,

    pub hits: Hits,
}
