use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::view::east::ec::ec;
use crate::view::east::en::en;
use crate::view::east::es::es;

pub fn area(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(area);

    en(f, chunks[0]);
    ec(f, chunks[1]);
    es(f, chunks[2]);
}
