use steel::SteelVal;

pub fn form(val: &SteelVal) -> String {
    match val {
        SteelVal::BoolV(b) => format!("{}", b),
        SteelVal::NumV(n) => format!("{}", n),
        SteelVal::IntV(i) => format!("{}", i),
        SteelVal::CharV(c) => format!("'{}'", c),
        SteelVal::StringV(s) => format!("\"{}\"", s),
        SteelVal::SymbolV(s) => format!("{}", s),
        SteelVal::ListV(l) => {
            let items: Vec<String> = l.iter().map(form).collect();
            format!("({})", items.join(" "))
        }
        SteelVal::VectorV(v) => {
            let items: Vec<String> = v.iter().map(form).collect();
            format!("#({})", items.join(" "))
        }
        SteelVal::HashMapV(m) => {
            let entries: Vec<String> = m
                .iter()
                .map(|(k, v)| format!("{}: {}", form(k), form(v)))
                .collect();
            format!("#hash({})", entries.join(", "))
        }
        SteelVal::HashSetV(s) => {
            let items: Vec<String> = s.iter().map(form).collect();
            format!("#set({})", items.join(" "))
        }
        SteelVal::Void => "#<void>".to_string(),
        SteelVal::Closure(_) => "#<closure>".to_string(),
        SteelVal::FuncV(_) => "#<builtin>".to_string(),
        _ => format!("{:?}", val),
    }
}
