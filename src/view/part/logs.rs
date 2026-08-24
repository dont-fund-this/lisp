use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::Frame;

use crate::core::App;
use crate::view::tint::{ACC, TXT};

pub fn logs(f: &mut Frame, area: Rect, block: Block, app: &App) {
    let mut lines = Vec::new();
    if let Some(ref res) = app.pane.res {
        if res.err {
            lines.push(Line::from(Span::styled("Status: ERROR", ratatui::style::Style::default().fg(ACC))));
            if let Some(ref e) = res.msg {
                for l in e.lines() {
                    lines.push(Line::from(Span::styled(format!("  {}", l), ratatui::style::Style::default().fg(TXT))));
                }
            }
        } else {
            lines.push(Line::from(Span::styled("Status: SUCCESS", ratatui::style::Style::default().fg(ACC))));
            lines.push(Line::from(Span::styled(format!("Execution time: {:.3}ms", res.dur.as_secs_f64() * 1000.0), ratatui::style::Style::default().fg(TXT))));
        }
    }
    f.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), area);
}
