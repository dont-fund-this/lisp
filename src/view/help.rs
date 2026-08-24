use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::view::tint::{ACC, BAR};

pub fn help(f: &mut Frame, area: Rect) {
    let pop = pop_r(50, 50, area);
    f.render_widget(Clear, pop);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(ACC))
        .title(Span::styled(" Shortcuts ", Style::default().fg(ACC)))
        .style(Style::default().bg(BAR));

    let txt = vec![
        Line::from(Span::styled("Clicks:", Style::default().fg(ACC))),
        Line::from("  • Click [▶ RUN]        : Exec query"),
        Line::from("  • Click [↻ RESET]      : Reset VM"),
        Line::from("  • Click [+]            : New tab"),
        Line::from("  • Click tabs           : Switch tab"),
        Line::from("  • Click [Table/Logs]   : Switch view"),
        Line::from("  • Drag splitters       : Resize panes"),
        Line::raw(""),
        Line::from(Span::styled("Keys (Ctrl+):", Style::default().fg(ACC))),
        Line::from("  Ctrl+E / Ctrl+Enter    : Exec query"),
        Line::from("  Ctrl+R                 : Reset VM"),
        Line::from("  Tab / Shift+Tab        : Cycle focus"),
        Line::from("  Ctrl+T / Ctrl+W        : New / Close tab"),
        Line::from("  Ctrl+B                 : Toggle Tree"),
        Line::from("  Ctrl+H                 : Help"),
        Line::from("  Ctrl+Q                 : Quit"),
    ];

    f.render_widget(Paragraph::new(txt).block(block).alignment(Alignment::Left), pop);
}

fn pop_r(px: u16, py: u16, r: Rect) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(v[1])[1]
}
