use crate::eval::Res;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Grid = 0,
    Logs = 1,
}

pub struct Pane {
    pub mode: Mode,
    pub res: Option<Res>,
    pub row: usize,
    pub col: usize,
}

impl Pane {
    pub fn new() -> Self {
        Self {
            mode: Mode::Grid,
            res: None,
            row: 0,
            col: 0,
        }
    }
}
