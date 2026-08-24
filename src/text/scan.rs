use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

const COL_KW: Color = Color::Rgb(212, 163, 115);
const COL_MUT: Color = Color::Rgb(148, 163, 184);
const COL_TXT: Color = Color::Rgb(241, 245, 249);

pub fn scan<'a>(raw: &'a str) -> Line<'a> {
    let mut spans = Vec::new();
    let mut chars = raw.char_indices().peekable();

    let kws = [
        "define", "defn", "lambda", "fn", "let", "let*", "if", "cond", "match",
        "struct", "filter", "map", "foldl", "foldr", "hash", "hash-ref", "list",
    ];

    while let Some(&(idx, c)) = chars.peek() {
        if c == ';' {
            spans.push(Span::styled(&raw[idx..], Style::default().fg(COL_MUT)));
            break;
        } else if c == '"' {
            let start = idx;
            chars.next();
            while let Some(&(_, next_c)) = chars.peek() {
                chars.next();
                if next_c == '"' {
                    break;
                }
            }
            let end = chars.peek().map(|&(i, _)| i).unwrap_or(raw.len());
            spans.push(Span::styled(&raw[start..end], Style::default().fg(COL_KW)));
        } else if c == '(' || c == ')' || c == '[' || c == ']' {
            chars.next();
            spans.push(Span::styled(&raw[idx..idx + c.len_utf8()], Style::default().fg(COL_MUT)));
        } else if c.is_whitespace() {
            let start = idx;
            while let Some(&(_, next_c)) = chars.peek() {
                if next_c.is_whitespace() {
                    chars.next();
                } else {
                    break;
                }
            }
            let end = chars.peek().map(|&(i, _)| i).unwrap_or(raw.len());
            spans.push(Span::raw(&raw[start..end]));
        } else {
            let start = idx;
            while let Some(&(_, next_c)) = chars.peek() {
                if next_c.is_whitespace() || "()[]\";".contains(next_c) {
                    break;
                }
                chars.next();
            }
            let end = chars.peek().map(|&(i, _)| i).unwrap_or(raw.len());
            let tok = &raw[start..end];

            let sty = if kws.contains(&tok) {
                Style::default().fg(COL_KW).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(COL_TXT)
            };
            spans.push(Span::styled(tok, sty));
        }
    }

    Line::from(spans)
}
