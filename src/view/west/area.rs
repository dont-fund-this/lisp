use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::App;
use crate::view::west::wc::wc;
use crate::view::west::wn::wn;
use crate::view::west::ws::ws;

pub fn area(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(area);

    wn(f, chunks[0], app);
    wc(f, chunks[1], app);
    ws(f, chunks[2]);
}
