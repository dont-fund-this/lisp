use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::{App, Focus};
use crate::view::tint::{ACC, BG};

pub fn sw(f: &mut Frame, area: Rect, app: &App) {
    let tag = match app.focus {
        Focus::Tree => " 📁 ",
        Focus::Text => " 📄 ",
        Focus::Pane => " 📊 ",
    };
    let span = Span::styled(
        tag,
        Style::default()
            .fg(BG)
            .bg(ACC)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(Paragraph::new(Line::from(span)).style(Style::default().bg(ACC)), area);
}
