use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::view::tint::{ACC, BG};

pub fn se(f: &mut Frame, area: Rect) {
    let size_str = std::env::current_exe()
        .ok()
        .and_then(|p| p.metadata().ok())
        .map(|m| format!(" {:.1}M ", m.len() as f64 / 1_048_576.0))
        .unwrap_or_else(|| " 5.2M ".to_string());

    let span = Span::styled(
        size_str,
        Style::default()
            .fg(BG)
            .bg(ACC)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(Paragraph::new(Line::from(span)).style(Style::default().bg(ACC)), area);
}
