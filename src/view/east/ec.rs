use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::view::tint::{MUT, PAN};

pub fn ec(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(" ▤ ", Style::default().fg(MUT).bg(PAN))),
        Line::raw(""),
        Line::from(Span::styled(" ⌘ ", Style::default().fg(MUT).bg(PAN))),
    ];
    f.render_widget(Paragraph::new(lines).style(Style::default().bg(PAN)), area);
}
