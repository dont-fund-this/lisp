use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::view::tint::{ACC, PAN};

pub fn es(f: &mut Frame, area: Rect) {
    let line = Line::from(vec![Span::styled(
        " ? ",
        Style::default()
            .fg(ACC)
            .bg(PAN)
            .add_modifier(Modifier::BOLD),
    )]);
    f.render_widget(Paragraph::new(vec![line]).style(Style::default().bg(PAN)), area);
}
