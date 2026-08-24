use ratatui::layout::{Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::core::{App, Focus};
use crate::text::scan;
use crate::view::tint::{ACC, BAR, BG, MUT};
use crate::view::r#type::Hits;

pub fn text(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    let is_foc = app.focus == Focus::Text;
    let bcol = if is_foc { ACC } else { MUT };

    let tab = app.text.tab();
    let num_lines = tab.lines.len();
    let num_w = format!("{}", num_lines.max(99)).len() + 1;
    hits.num_w = num_w;
    hits.text = area;

    let vh = area.height.saturating_sub(2) as usize;
    let start = tab.off;
    let end = (start + vh).min(tab.lines.len());

    let mut out_lines = Vec::new();
    for (idx, raw) in tab.lines[start..end].iter().enumerate() {
        let lnum = start + idx + 1;
        let is_cur = lnum - 1 == tab.row && is_foc;

        let num_sty = if is_cur { Style::default().fg(ACC) } else { Style::default().fg(MUT) };
        let mut spans = vec![Span::styled(format!("{:>width$} | ", lnum, width = num_w), num_sty)];

        let highlighted = scan(raw);
        spans.extend(highlighted.spans);

        let line_sty = if is_cur { Style::default().bg(BAR) } else { Style::default() };
        out_lines.push(Line::from(spans).style(line_sty));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(bcol))
        .title(Span::styled(format!(" 1. Query: {} ", tab.name), Style::default().fg(bcol).add_modifier(Modifier::BOLD)))
        .style(Style::default().bg(BG));

    f.render_widget(Paragraph::new(out_lines).block(block), area);

    if is_foc && tab.row >= start && tab.row < end {
        let cur_x = area.x + 1 + num_w as u16 + 3 + tab.col as u16;
        let cur_y = area.y + 1 + (tab.row - start) as u16;
        if cur_x < area.x + area.width.saturating_sub(1) && cur_y < area.y + area.height.saturating_sub(1) {
            f.set_cursor_position(Position::new(cur_x, cur_y));
        }
    }
}
