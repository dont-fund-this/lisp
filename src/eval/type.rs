use std::time::Duration;
use steel::steel_vm::engine::Engine;

#[derive(Debug, Clone)]
pub struct Col {
    pub name: String,
    pub w: usize,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub cols: Vec<Col>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Res {
    pub vals: Vec<String>,
    pub grid: Option<Grid>,
    pub dur: Duration,
    pub err: bool,
    pub msg: Option<String>,
}

pub struct Vm {
    pub eng: Engine,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            eng: Engine::new(),
        }
    }

    pub fn rset(&mut self) {
        self.eng = Engine::new();
    }
}
