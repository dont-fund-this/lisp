use crate::tree::r#type::{Item, Nav};

pub fn look(nav: &Nav) -> Option<&Item> {
    nav.list.get(nav.sel)
}
