use crate::eval::Res;
use crate::pane::r#type::{Mode, Pane};

pub fn show(pane: &mut Pane, res: Res) {
    pane.mode = if res.err { Mode::Logs } else { Mode::Grid };
    pane.res = Some(res);
    pane.row = 0;
    pane.col = 0;
}
