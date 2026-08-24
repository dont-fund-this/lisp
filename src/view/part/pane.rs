use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;

use crate::core::{App, Focus};
use crate::pane::Mode;
use crate::view::part::grid::grid;
use crate::view::part::logs::logs;
use crate::view::tint::{ACC, BG, MUT};
use crate::view::r#type::Hits;

pub fn pane(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    let is_foc = app.focus == Focus::Pane;
    let bcol = if is_foc { ACC } else { MUT };
    hits.pane = area;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(bcol))
        .title(Span::styled(" 2. Results ", Style::default().fg(bcol)))
        .style(Style::default().bg(BG));

    match app.pane.mode {
        Mode::Grid => grid(f, area, block, app),
        Mode::Logs => logs(f, area, block, app),
    }
}
