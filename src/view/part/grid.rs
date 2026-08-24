use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

use crate::core::{App, Focus};
use crate::view::tint::{ACC, BG, MUT, TXT};

pub fn grid(f: &mut Frame, area: Rect, block: Block, app: &App) {
    if let Some(ref res) = app.pane.res {
        if res.err {
            let err_line = Line::from(vec![
                Span::styled("Error: ", Style::default().fg(ACC)),
                Span::styled(res.msg.as_deref().unwrap_or("Unknown error"), Style::default().fg(TXT)),
            ]);
            f.render_widget(Paragraph::new(vec![Line::raw(""), err_line]).block(block), area);
            return;
        }

        if let Some(ref g) = res.grid {
            let mut lines = Vec::new();

            let mut hdr = vec![Span::styled(" # | ", Style::default().fg(MUT))];
            for col in &g.cols {
                hdr.push(Span::styled(format!("{:<width$} | ", col.name, width = col.w), Style::default().fg(ACC).add_modifier(Modifier::BOLD)));
            }
            lines.push(Line::from(hdr));

            let mut sep = vec![Span::styled("---+", Style::default().fg(MUT))];
            for col in &g.cols {
                sep.push(Span::styled(format!("{}-+-", "-".repeat(col.w)), Style::default().fg(MUT)));
            }
            lines.push(Line::from(sep));

            for (r_idx, row) in g.rows.iter().enumerate() {
                let is_sel = r_idx == app.pane.row;
                let mut row_spans = vec![Span::styled(format!("{:>2} | ", r_idx + 1), Style::default().fg(MUT))];

                for (c_idx, val) in row.iter().enumerate() {
                    let cw = g.cols.get(c_idx).map(|c| c.w).unwrap_or(12);
                    let is_cell = is_sel && c_idx == app.pane.col && app.focus == Focus::Pane;

                    let sty = if is_cell {
                        Style::default().fg(BG).bg(ACC)
                    } else if is_sel {
                        Style::default().fg(ACC)
                    } else {
                        Style::default().fg(TXT)
                    };

                    row_spans.push(Span::styled(format!("{:<width$} | ", val, width = cw), sty));
                }

                lines.push(Line::from(row_spans));
            }

            lines.push(Line::raw(""));
            lines.push(Line::from(vec![Span::styled(format!("  {} rows in dataset | ⏱ {:.3}ms", g.rows.len(), res.dur.as_secs_f64() * 1000.0), Style::default().fg(MUT))]));

            f.render_widget(Paragraph::new(lines).block(block), area);
            return;
        }

        let mut lines = Vec::new();
        for val_str in &res.vals {
            lines.push(Line::from(vec![Span::styled("  ▶ ", Style::default().fg(ACC)), Span::styled(val_str, Style::default().fg(TXT))]));
        }
        f.render_widget(Paragraph::new(lines).block(block), area);
    } else {
        f.render_widget(Paragraph::new("").block(block), area);
    }
}
