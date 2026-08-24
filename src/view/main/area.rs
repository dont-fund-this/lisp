use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::App;
use crate::view::main::body::body;
use crate::view::main::foot::foot;
use crate::view::main::head::head;
use crate::view::r#type::Hits;

pub fn area(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(area);

    head(f, chunks[0], app, hits);
    body(f, chunks[1], app, hits);
    foot(f, chunks[2], app);
}
