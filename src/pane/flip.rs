use crate::pane::r#type::{Mode, Pane};

pub fn flip(pane: &mut Pane) {
    pane.mode = match pane.mode {
        Mode::Grid => Mode::Logs,
        Mode::Logs => Mode::Grid,
    };
}
