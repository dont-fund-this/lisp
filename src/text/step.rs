use crate::text::r#type::Tab;

pub fn go_l(tab: &mut Tab) {
    if tab.col > 0 {
        tab.col -= 1;
    } else if tab.row > 0 {
        tab.row -= 1;
        tab.col = tab.lines[tab.row].len();
    }
}

pub fn go_r(tab: &mut Tab) {
    if tab.row < tab.lines.len() {
        let len = tab.lines[tab.row].len();
        if tab.col < len {
            tab.col += 1;
        } else if tab.row + 1 < tab.lines.len() {
            tab.row += 1;
            tab.col = 0;
        }
    }
}

pub fn go_u(tab: &mut Tab) {
    if tab.row > 0 {
        tab.row -= 1;
        clamp(tab);
    }
}

pub fn go_d(tab: &mut Tab) {
    if tab.row + 1 < tab.lines.len() {
        tab.row += 1;
        clamp(tab);
    }
}

pub fn go_sol(tab: &mut Tab) {
    tab.col = 0;
}

pub fn go_eol(tab: &mut Tab) {
    if tab.row < tab.lines.len() {
        tab.col = tab.lines[tab.row].len();
    }
}

pub fn pg_u(tab: &mut Tab, n: usize) {
    tab.row = tab.row.saturating_sub(n);
    clamp(tab);
}

pub fn pg_d(tab: &mut Tab, n: usize) {
    tab.row = (tab.row + n).min(tab.lines.len().saturating_sub(1));
    clamp(tab);
}

fn clamp(tab: &mut Tab) {
    if tab.row >= tab.lines.len() {
        tab.row = tab.lines.len().saturating_sub(1);
    }
    let len = tab.lines.get(tab.row).map(|l| l.len()).unwrap_or(0);
    if tab.col > len {
        tab.col = len;
    }
}
