use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::view::tint::{MUT, PAN};

pub fn en(f: &mut Frame, area: Rect) {
    let line = Line::from(vec![Span::styled(" ⚙ ", Style::default().fg(MUT).bg(PAN))]);
    f.render_widget(Paragraph::new(vec![line]).style(Style::default().bg(PAN)), area);
}
