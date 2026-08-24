#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Scm,
    Sym(String),
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub kind: Kind,
}

pub struct Nav {
    pub list: Vec<Item>,
    pub sel: usize,
}

impl Nav {
    pub fn new() -> Self {
        let mut list = vec![Item {
            name: "thingy.scm".to_string(),
            kind: Kind::Scm,
        }];

        for s in [
            "filter", "map", "foldl", "foldr", "hash", "list", "define", "struct",
        ] {
            list.push(Item {
                name: s.to_string(),
                kind: Kind::Sym(s.to_string()),
            });
        }

        Self { list, sel: 0 }
    }
}
