use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::view::tint::{ACC, BG};

pub fn ne(f: &mut Frame, area: Rect) {
    let span = Span::styled(
        " ⯇ ",
        Style::default()
            .fg(BG)
            .bg(ACC)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(Paragraph::new(Line::from(span)).style(Style::default().bg(ACC)), area);
}
