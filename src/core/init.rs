use crate::core::exec::exec;
use crate::core::r#type::{App, Focus};
use crate::eval::Vm;
use crate::pane::Pane;
use crate::text::Book;
use crate::tree::Nav;
use crate::view::Hits;

pub fn init() -> App {
    let mut a = App {
        vm: Vm::new(),
        text: Book::new(),
        tree: Nav::new(),
        pane: Pane::new(),
        focus: Focus::Text,
        show_tree: true,
        show_help: false,
        should_quit: false,
        tree_w: 22,
        split_pct: 50,
        drag_v: false,
        drag_h: false,
        hits: Hits::default(),
    };
    exec(&mut a);
    a
}
