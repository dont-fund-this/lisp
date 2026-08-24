use std::collections::BTreeMap;
use steel::SteelVal;

use crate::eval::form::form;
use crate::eval::r#type::{Col, Grid};

pub fn pick(val: &SteelVal) -> Option<Grid> {
    match val {
        SteelVal::ListV(list) if !list.is_empty() => {
            let items: Vec<&SteelVal> = list.iter().collect();
            from_items(&items)
        }
        SteelVal::VectorV(vec) if !vec.is_empty() => {
            let items: Vec<&SteelVal> = vec.iter().collect();
            from_items(&items)
        }
        SteelVal::HashMapV(map) if !map.is_empty() => {
            let mut cols = vec![
                Col { name: "Key".to_string(), w: 12 },
                Col { name: "Val".to_string(), w: 24 },
            ];
            let mut rows = Vec::new();
            for (k, v) in map.iter() {
                let k_str = form(k).trim_matches('"').to_string();
                let v_str = form(v);
                cols[0].w = cols[0].w.max(k_str.len() + 2);
                cols[1].w = cols[1].w.max(v_str.len() + 2);
                rows.push(vec![k_str, v_str]);
            }
            Some(Grid { cols, rows })
        }
        _ => None,
    }
}

fn from_items(items: &[&SteelVal]) -> Option<Grid> {
    let first = items.first()?;
    if let SteelVal::HashMapV(_) = first {
        let mut keys = BTreeMap::new();
        for &item in items {
            if let SteelVal::HashMapV(m) = item {
                for (k, _) in m.iter() {
                    keys.insert(form(k).trim_matches('"').to_string(), ());
                }
            }
        }

        let mut cols: Vec<Col> = keys
            .keys()
            .map(|k| Col {
                name: k.clone(),
                w: k.len().max(8) + 2,
            })
            .collect();

        let mut rows = Vec::new();
        for &item in items {
            if let SteelVal::HashMapV(m) = item {
                let mut row = Vec::new();
                for col in cols.iter_mut() {
                    let mut found = false;
                    for (k, v) in m.iter() {
                        let k_str = form(k).trim_matches('"').to_string();
                        if k_str == col.name {
                            let v_str = form(v).trim_matches('"').to_string();
                            col.w = col.w.max(v_str.len() + 2);
                            row.push(v_str);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        row.push("NULL".to_string());
                    }
                }
                rows.push(row);
            }
        }
        Some(Grid { cols, rows })
    } else {
        let mut col = Col { name: "Val".to_string(), w: 12 };
        let mut rows = Vec::new();
        for &item in items {
            let v_str = form(item).trim_matches('"').to_string();
            col.w = col.w.max(v_str.len() + 2);
            rows.push(vec![v_str]);
        }
        Some(Grid { cols: vec![col], rows })
    }
}
