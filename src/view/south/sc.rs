use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::{App, Focus};
use crate::view::tint::{ACC, BAR, BG, MUT};

pub fn sc(f: &mut Frame, area: Rect, app: &App) {
    let tag = match app.focus {
        Focus::Tree => "Tree",
        Focus::Text => "Text",
        Focus::Pane => "Pane",
    };

    let spans = vec![
        Span::styled(
            format!(" [{}] ", tag),
            Style::default()
                .fg(BG)
                .bg(ACC)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            "[Tab] Focus  │  [Ctrl+E] Run  │  [Ctrl+R] Reset  │  [Ctrl+T] Tab  │  [Ctrl+Q] Quit",
            Style::default().fg(MUT),
        ),
    ];

    f.render_widget(Paragraph::new(Line::from(spans)).style(Style::default().bg(BAR)), area);
}
