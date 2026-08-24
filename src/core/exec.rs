use crate::core::r#type::App;
use crate::eval::eval;
use crate::pane::show;

pub fn exec(app: &mut App) {
    let code = app.text.tab().text();
    let res = eval(&mut app.vm, &code);
    show(&mut app.pane, res);
}
