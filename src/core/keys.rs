use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::code::CODE;
use crate::core::exec::exec;
use crate::core::rset::rset;
use crate::core::r#type::{App, Focus};
use crate::pane::{flip, Mode};
use crate::text::edit::{del_l, del_r, put_c, put_nl};
use crate::text::step::{go_d, go_eol, go_l, go_r, go_sol, go_u, pg_d, pg_u};
use crate::tree::look;
use crate::tree::next::next;
use crate::tree::prev::prev;
use crate::tree::r#type::Kind;

pub fn keys(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL)
        && (key.code == KeyCode::Char('c') || key.code == KeyCode::Char('q'))
    {
        app.should_quit = true;
        return;
    }

    if app.show_help {
        if key.code == KeyCode::Esc
            || key.code == KeyCode::Char('q')
            || (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('h'))
        {
            app.show_help = false;
        }
        return;
    }

    if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('h'))
        || (key.code == KeyCode::Char('?') && app.focus != Focus::Text)
    {
        app.show_help = true;
        return;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL)
        && (key.code == KeyCode::Char('e') || key.code == KeyCode::Enter)
    {
        exec(app);
        return;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('r') {
        rset(app);
        return;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('b') {
        app.show_tree = !app.show_tree;
        if !app.show_tree && app.focus == Focus::Tree {
            app.focus = Focus::Text;
        }
        return;
    }

    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            app.focus = match app.focus {
                Focus::Tree => Focus::Pane,
                Focus::Text => {
                    if app.show_tree { Focus::Tree } else { Focus::Pane }
                }
                Focus::Pane => Focus::Text,
            };
        } else {
            app.focus = match app.focus {
                Focus::Tree => Focus::Text,
                Focus::Text => Focus::Pane,
                Focus::Pane => {
                    if app.show_tree { Focus::Tree } else { Focus::Text }
                }
            };
        }
        return;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('t') {
        let num = app.text.tabs.len() + 1;
        app.text.add(format!("Query{}.scm", num), "");
        app.focus = Focus::Text;
        return;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('w') {
        app.text.del();
        return;
    }

    if key.code == KeyCode::PageUp && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.text.prev();
        return;
    }
    if key.code == KeyCode::PageDown && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.text.next();
        return;
    }

    match app.focus {
        Focus::Tree => match key.code {
            KeyCode::Up | KeyCode::Char('k') => prev(&mut app.tree),
            KeyCode::Down | KeyCode::Char('j') => next(&mut app.tree),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(item) = look(&app.tree).cloned() {
                    match item.kind {
                        Kind::Scm => {
                            app.text.add(item.name, CODE);
                            app.focus = Focus::Text;
                        }
                        Kind::Sym(sym) => {
                            app.text.add(format!("{}.scm", sym), &format!("{}\n", sym));
                            app.focus = Focus::Text;
                        }
                    }
                }
            }
            _ => {}
        },
        Focus::Text => {
            let tab = app.text.tab_mut();
            match key.code {
                KeyCode::Left => go_l(tab),
                KeyCode::Right => go_r(tab),
                KeyCode::Up => go_u(tab),
                KeyCode::Down => go_d(tab),
                KeyCode::Home => go_sol(tab),
                KeyCode::End => go_eol(tab),
                KeyCode::PageUp => pg_u(tab, 10),
                KeyCode::PageDown => pg_d(tab, 10),
                KeyCode::Enter => put_nl(tab),
                KeyCode::Backspace => del_l(tab),
                KeyCode::Delete => del_r(tab),
                KeyCode::Char(c) => {
                    if !key.modifiers.contains(KeyModifiers::CONTROL) {
                        put_c(tab, c);
                    }
                }
                _ => {}
            }
        }
        Focus::Pane => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.pane.mode == Mode::Grid && app.pane.row > 0 {
                    app.pane.row -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.pane.mode == Mode::Grid {
                    if let Some(ref r) = app.pane.res {
                        if let Some(ref g) = r.grid {
                            if app.pane.row + 1 < g.rows.len() {
                                app.pane.row += 1;
                            }
                        }
                    }
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if app.pane.mode == Mode::Grid && app.pane.col > 0 {
                    app.pane.col -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if app.pane.mode == Mode::Grid {
                    if let Some(ref r) = app.pane.res {
                        if let Some(ref g) = r.grid {
                            if app.pane.col + 1 < g.cols.len() {
                                app.pane.col += 1;
                            }
                        }
                    }
                }
            }
            KeyCode::Char('1') => app.pane.mode = Mode::Grid,
            KeyCode::Char('2') => app.pane.mode = Mode::Logs,
            KeyCode::Tab => flip(&mut app.pane),
            _ => {}
        },
    }
}
