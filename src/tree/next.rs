use crate::tree::r#type::Nav;

pub fn next(nav: &mut Nav) {
    if nav.sel + 1 < nav.list.len() {
        nav.sel += 1;
    }
}
