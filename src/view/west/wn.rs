use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::App;
use crate::view::tint::{ACC, MUT, PAN};

pub fn wn(f: &mut Frame, area: Rect, app: &App) {
    let sty = if app.show_tree {
        Style::default().fg(ACC).bg(PAN)
    } else {
        Style::default().fg(MUT).bg(PAN)
    };
    let line = Line::from(vec![Span::styled(" 📁 ", sty)]);
    f.render_widget(Paragraph::new(vec![line]).style(Style::default().bg(PAN)), area);
}
