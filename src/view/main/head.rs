use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::core::App;
use crate::pane::Mode;
use crate::view::tint::{ACC, BAR, BG, MUT, TXT};
use crate::view::r#type::Hits;

pub fn head(f: &mut Frame, area: Rect, app: &App, hits: &mut Hits) {
    let mut spans = Vec::new();
    let mut cx = area.x;

    let run_str = " ▶ RUN ";
    let run_w = run_str.width() as u16;
    spans.push(Span::styled(run_str, Style::default().fg(BG).bg(ACC).add_modifier(Modifier::BOLD)));
    hits.run = Rect::new(cx, area.y, run_w, 1);
    cx += run_w + 1;
    spans.push(Span::raw(" "));

    let rst_str = " ↻ RESET ";
    let rst_w = rst_str.width() as u16;
    spans.push(Span::styled(rst_str, Style::default().fg(TXT).bg(BAR).add_modifier(Modifier::BOLD)));
    hits.rset = Rect::new(cx, area.y, rst_w, 1);
    cx += rst_w + 3;
    spans.push(Span::raw(" │ "));

    for (i, tab) in app.text.tabs.iter().enumerate() {
        let is_cur = i == app.text.cur;
        let title = format!(" {} ", tab.name);
        let tw = title.width() as u16;
        hits.tabs.push((Rect::new(cx, area.y, tw, 1), i));
        cx += tw + 1;

        if is_cur {
            spans.push(Span::styled(title, Style::default().fg(ACC).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)));
        } else {
            spans.push(Span::styled(title, Style::default().fg(MUT)));
        }
        spans.push(Span::raw(" "));
    }

    let new_str = " [+] ";
    let nw = new_str.width() as u16;
    spans.push(Span::styled(new_str, Style::default().fg(ACC).bg(BAR).add_modifier(Modifier::BOLD)));
    hits.new = Rect::new(cx, area.y, nw, 1);
    cx += nw + 3;
    spans.push(Span::raw(" │ "));

    let is_g = app.pane.mode == Mode::Grid;
    let g_str = " [Table Grid] ";
    let gw = g_str.width() as u16;
    hits.modes.push((Rect::new(cx, area.y, gw, 1), Mode::Grid));
    cx += gw + 1;

    spans.push(Span::styled(g_str, if is_g { Style::default().fg(BG).bg(ACC).add_modifier(Modifier::BOLD) } else { Style::default().fg(MUT) }));
    spans.push(Span::raw(" "));

    let m_str = " [Logs] ";
    let mw = m_str.width() as u16;
    hits.modes.push((Rect::new(cx, area.y, mw, 1), Mode::Logs));

    spans.push(Span::styled(m_str, if !is_g { Style::default().fg(BG).bg(ACC).add_modifier(Modifier::BOLD) } else { Style::default().fg(MUT) }));

    f.render_widget(Paragraph::new(Line::from(spans)).style(Style::default().bg(BAR)), area);
}
