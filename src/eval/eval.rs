use std::time::Instant;

use crate::eval::form::form;
use crate::eval::pick::pick;
use crate::eval::r#type::{Res, Vm};

pub fn eval(vm: &mut Vm, code: &str) -> Res {
    let t0 = Instant::now();
    let res = vm.eng.compile_and_run_raw_program(code.to_string());
    let dur = t0.elapsed();

    match res {
        Ok(values) => {
            let vals: Vec<String> = values.iter().map(form).collect();
            let grid = values.last().and_then(pick);

            Res {
                vals,
                grid,
                dur,
                err: false,
                msg: None,
            }
        }
        Err(e) => Res {
            vals: Vec::new(),
            grid: None,
            dur,
            err: true,
            msg: Some(format!("{}", e)),
        },
    }
}
