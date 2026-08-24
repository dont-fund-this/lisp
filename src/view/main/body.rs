use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::App;
use crate::view::part::pane;
use crate::view::part::text;
use crate::view::part::tree;
use crate::view::r#type::Hits;

pub fn body(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    if app.show_tree {
        let sw = app.tree_w.min(area.width.saturating_sub(30));
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(sw), Constraint::Min(30)])
            .split(area);

        hits.v_bar = Rect::new(h_chunks[0].x + h_chunks[0].width.saturating_sub(1), h_chunks[0].y, 2, h_chunks[0].height);

        tree(f, h_chunks[0], app, hits);
        split(f, h_chunks[1], app, hits);
    } else {
        split(f, area, app, hits);
    }
}

fn split(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    let pct = app.split_pct.clamp(15, 85);
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(pct), Constraint::Percentage(100 - pct)])
        .split(area);

    hits.h_bar = Rect::new(v_chunks[0].x, v_chunks[0].y + v_chunks[0].height.saturating_sub(1), v_chunks[0].width, 2);

    text(f, v_chunks[0], app, hits);
    pane(f, v_chunks[1], app, hits);
}
