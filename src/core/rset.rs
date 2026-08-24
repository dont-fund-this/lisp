use crate::core::r#type::App;
use crate::eval::eval;
use crate::pane::show;

pub fn rset(app: &mut App) {
    app.vm.rset();
    let res = eval(&mut app.vm, "");
    show(&mut app.pane, res);
}
