use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::App;
use crate::view::south::sc::sc;
use crate::view::south::se::se;
use crate::view::south::sw::sw;

pub fn area(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(5),
        ])
        .split(area);

    sw(f, chunks[0], app);
    sc(f, chunks[1], app);
    se(f, chunks[2]);
}
