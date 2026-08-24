use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::core::{App, Focus};
use crate::view::tint::{ACC, BG, MUT, TXT};
use crate::view::r#type::Hits;

pub fn tree(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    hits.tree = area;
    let is_foc = app.focus == Focus::Tree;
    let bcol = if is_foc { ACC } else { MUT };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(bcol))
        .title(Span::styled(" Tree ", Style::default().fg(bcol)))
        .style(Style::default().bg(BG));

    let mut lines = Vec::new();
    let start_y = area.y + 1;

    for (i, item) in app.tree.list.iter().enumerate() {
        let is_sel = i == app.tree.sel;
        let sty = if is_sel && is_foc {
            Style::default().fg(BG).bg(ACC).add_modifier(Modifier::BOLD)
        } else if is_sel {
            Style::default().fg(ACC)
        } else {
            Style::default().fg(TXT)
        };

        if (start_y + i as u16) < area.y + area.height.saturating_sub(1) {
            let ir = Rect::new(area.x + 1, start_y + i as u16, area.width.saturating_sub(2), 1);
            hits.items.push((ir, i));
        }

        lines.push(Line::from(vec![Span::raw("  "), Span::styled(&item.name, sty)]));
    }

    f.render_widget(Paragraph::new(lines).block(block), area);
}
