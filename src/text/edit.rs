use crate::text::r#type::Tab;

pub fn put_c(tab: &mut Tab, c: char) {
    if tab.row >= tab.lines.len() {
        tab.lines.push(String::new());
        tab.row = tab.lines.len() - 1;
    }
    let line = &mut tab.lines[tab.row];
    if tab.col > line.len() {
        tab.col = line.len();
    }
    line.insert(tab.col, c);
    tab.col += 1;
}

pub fn put_nl(tab: &mut Tab) {
    if tab.row >= tab.lines.len() {
        tab.lines.push(String::new());
        tab.row = tab.lines.len() - 1;
    }
    let cur = &tab.lines[tab.row];
    let col = tab.col.min(cur.len());
    let ind_len = cur.chars().take_while(|c| *c == ' ').count();
    let ind = " ".repeat(ind_len);
    let rem = cur[col..].to_string();
    tab.lines[tab.row].truncate(col);
    tab.row += 1;
    tab.col = ind_len;
    tab.lines.insert(tab.row, format!("{}{}", ind, rem));
}

pub fn del_l(tab: &mut Tab) {
    if tab.row >= tab.lines.len() {
        return;
    }
    if tab.col > 0 {
        let line = &mut tab.lines[tab.row];
        if tab.col <= line.len() {
            line.remove(tab.col - 1);
            tab.col -= 1;
        }
    } else if tab.row > 0 {
        let cur = tab.lines.remove(tab.row);
        tab.row -= 1;
        let prev = &mut tab.lines[tab.row];
        tab.col = prev.len();
        prev.push_str(&cur);
    }
}

pub fn del_r(tab: &mut Tab) {
    if tab.row >= tab.lines.len() {
        return;
    }
    let len = tab.lines[tab.row].len();
    if tab.col < len {
        tab.lines[tab.row].remove(tab.col);
    } else if tab.row + 1 < tab.lines.len() {
        let next = tab.lines.remove(tab.row + 1);
        tab.lines[tab.row].push_str(&next);
    }
}
