use crate::tree::r#type::Nav;

pub fn prev(nav: &mut Nav) {
    if nav.sel > 0 {
        nav.sel -= 1;
    }
}
