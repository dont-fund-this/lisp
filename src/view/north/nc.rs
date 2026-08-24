use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::App;
use crate::view::tint::{ACC, BAR, BG, MUT, TXT};

pub fn nc(f: &mut Frame, area: Rect, _app: &App) {
    let title = Span::styled(
        " STEEL STUDIO ",
        Style::default()
            .fg(BG)
            .bg(ACC)
            .add_modifier(Modifier::BOLD),
    );
    let engine = Span::styled(" Scheme / Lisp v0.8.3 ", Style::default().fg(TXT).bg(BAR));
    let hint = Span::styled(
        " [Ctrl+E] Run  [Ctrl+R] Reset  [Ctrl+H] Help  [Ctrl+Q] Quit ",
        Style::default().fg(MUT).bg(BAR),
    );

    let mut spans = vec![title, engine];
    let used = 14 + 22 + 57;
    if area.width > used {
        let pad = area.width.saturating_sub(used) as usize;
        spans.push(Span::styled(" ".repeat(pad), Style::default().bg(BAR)));
        spans.push(hint);
    }

    f.render_widget(Paragraph::new(Line::from(spans)).style(Style::default().bg(BAR)), area);
}
