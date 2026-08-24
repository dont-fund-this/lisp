use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Position;

use crate::code::CODE;
use crate::core::exec::exec;
use crate::core::rset::rset;
use crate::core::r#type::{App, Focus};
use crate::pane::Mode;
use crate::text::step::{go_d, go_u};
use crate::tree::look;
use crate::tree::next::next;
use crate::tree::prev::prev;
use crate::tree::r#type::Kind;

pub fn mice(app: &mut App, m: MouseEvent) {
    let x = m.column;
    let y = m.row;
    let pos = Position::new(x, y);

    match m.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if app.show_help {
                app.show_help = false;
                return;
            }

            if app.hits.v_bar.contains(pos) {
                app.drag_v = true;
                return;
            }
            if app.hits.h_bar.contains(pos) {
                app.drag_h = true;
                return;
            }

            if app.hits.run.contains(pos) {
                exec(app);
                return;
            }
            if app.hits.rset.contains(pos) {
                rset(app);
                return;
            }
            if app.hits.help.contains(pos) {
                app.show_help = true;
                return;
            }

            if app.hits.new.contains(pos) {
                let num = app.text.tabs.len() + 1;
                app.text.add(format!("Query{}.scm", num), "");
                app.focus = Focus::Text;
                return;
            }

            for (r, i) in &app.hits.tabs {
                if r.contains(pos) {
                    app.text.cur = *i;
                    app.focus = Focus::Text;
                    return;
                }
            }

            for (r, mode) in &app.hits.modes {
                if r.contains(pos) {
                    app.pane.mode = *mode;
                    app.focus = Focus::Pane;
                    return;
                }
            }

            if app.hits.tree.contains(pos) {
                app.focus = Focus::Tree;
                for (r, i) in &app.hits.items {
                    if r.contains(pos) {
                        app.tree.sel = *i;
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
                        return;
                    }
                }
                return;
            }

            if app.hits.text.contains(pos) {
                app.focus = Focus::Text;
                let dy = y.saturating_sub(app.hits.text.y) as usize;
                let tab = app.text.tab_mut();
                let target = tab.off + dy;
                if target < tab.lines.len() {
                    tab.row = target;
                    let len = tab.lines[target].len();
                    let pfx = app.hits.num_w + 3;
                    let dx = (x as usize).saturating_sub(app.hits.text.x as usize + pfx);
                    tab.col = dx.min(len);
                }
                return;
            }

            if app.hits.pane.contains(pos) {
                app.focus = Focus::Pane;
                if app.pane.mode == Mode::Grid {
                    let dy = y.saturating_sub(app.hits.pane.y) as usize;
                    if dy >= 2 {
                        let row = dy - 2;
                        if let Some(ref r) = app.pane.res {
                            if let Some(ref g) = r.grid {
                                if row < g.rows.len() {
                                    app.pane.row = row;
                                }
                            }
                        }
                    }
                }
                return;
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if app.drag_v {
                app.tree_w = x.clamp(14, 60);
            } else if app.drag_h {
                let total = app.hits.text.height + app.hits.pane.height + 4;
                if total > 0 {
                    let sy = app.hits.text.y.saturating_sub(1);
                    let ry = y.saturating_sub(sy);
                    let pct = ((ry as f32 / total as f32) * 100.0) as u16;
                    app.split_pct = pct.clamp(15, 85);
                }
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            app.drag_v = false;
            app.drag_h = false;
        }
        MouseEventKind::ScrollUp => {
            if app.hits.tree.contains(pos) {
                prev(&mut app.tree);
            } else if app.hits.text.contains(pos) {
                go_u(app.text.tab_mut());
            } else if app.hits.pane.contains(pos) {
                if app.pane.mode == Mode::Grid && app.pane.row > 0 {
                    app.pane.row -= 1;
                }
            }
        }
        MouseEventKind::ScrollDown => {
            if app.hits.tree.contains(pos) {
                next(&mut app.tree);
            } else if app.hits.text.contains(pos) {
                go_d(app.text.tab_mut());
            } else if app.hits.pane.contains(pos) {
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
        }
        _ => {}
    }
}
