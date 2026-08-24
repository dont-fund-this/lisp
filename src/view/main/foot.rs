use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::App;
use crate::view::tint::{ACC, BAR, MUT, TXT};

pub fn foot(f: &mut Frame, area: Rect, app: &App) {
    let tab = app.text.tab();
    let rows = if let Some(ref r) = app.pane.res {
        if let Some(ref g) = r.grid {
            format!("Rows: {}", g.rows.len())
        } else {
            format!("Vals: {}", r.vals.len())
        }
    } else {
        "Rows: 0".to_string()
    };

    let dur = if let Some(ref r) = app.pane.res {
        format!("{:.3}ms", r.dur.as_secs_f64() * 1000.0)
    } else {
        "0.000ms".to_string()
    };

    let spans = vec![
        Span::styled(format!(" 📄 {} ", tab.name), Style::default().fg(ACC).add_modifier(Modifier::BOLD)),
        Span::raw("│ "),
        Span::styled(format!("Ln {}, Col {} ", tab.row + 1, tab.col + 1), Style::default().fg(TXT)),
        Span::raw("│ "),
        Span::styled(format!("{} ", rows), Style::default().fg(TXT)),
        Span::raw("│ "),
        Span::styled(format!("⏱ {} ", dur), Style::default().fg(ACC)),
        Span::raw("│ "),
        Span::styled("UTF-8 │ Steel VM", Style::default().fg(MUT)),
    ];

    f.render_widget(Paragraph::new(Line::from(spans)).style(Style::default().bg(BAR)), area);
}
