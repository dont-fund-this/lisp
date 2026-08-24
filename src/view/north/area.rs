use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::App;
use crate::view::north::nc::nc;
use crate::view::north::ne::ne;
use crate::view::north::nw::nw;

pub fn area(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(3),
        ])
        .split(area);

    nw(f, chunks[0]);
    nc(f, chunks[1], app);
    ne(f, chunks[2]);
}
