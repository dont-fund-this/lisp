use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::App;
use crate::pane::Mode;
use crate::view::tint::{ACC, MUT, PAN};

pub fn wc(f: &mut Frame, area: Rect, app: &App) {
    let icons = [
        ("⚡", true),
        ("📊", app.pane.mode == Mode::Grid),
        ("💬", app.pane.mode == Mode::Logs),
    ];

    let mut lines = Vec::new();
    for (icon, on) in icons {
        let sty = if on {
            Style::default().fg(ACC).bg(PAN)
        } else {
            Style::default().fg(MUT).bg(PAN)
        };
        lines.push(Line::from(vec![Span::styled(format!(" {} ", icon), sty)]));
        lines.push(Line::raw(""));
    }

    f.render_widget(Paragraph::new(lines).style(Style::default().bg(PAN)), area);
}
