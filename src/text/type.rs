use crate::code::CODE;

#[derive(Debug, Clone)]
pub struct Tab {
    pub name: String,
    pub lines: Vec<String>,
    pub row: usize,
    pub col: usize,
    pub off: usize,
}

impl Tab {
    pub fn new(name: String, content: &str) -> Self {
        let lines = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };
        Self {
            name,
            lines,
            row: 0,
            col: 0,
            off: 0,
        }
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }
}

pub struct Book {
    pub tabs: Vec<Tab>,
    pub cur: usize,
}

impl Book {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::new("thingy.scm".to_string(), CODE)],
            cur: 0,
        }
    }

    pub fn tab(&self) -> &Tab {
        &self.tabs[self.cur]
    }

    pub fn tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.cur]
    }

    pub fn add(&mut self, name: String, text: &str) {
        self.tabs.push(Tab::new(name, text));
        self.cur = self.tabs.len() - 1;
    }

    pub fn del(&mut self) {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.cur);
            if self.cur >= self.tabs.len() {
                self.cur = self.tabs.len() - 1;
            }
        }
    }

    pub fn next(&mut self) {
        if !self.tabs.is_empty() {
            self.cur = (self.cur + 1) % self.tabs.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.tabs.is_empty() {
            self.cur = if self.cur == 0 {
                self.tabs.len() - 1
            } else {
                self.cur - 1
            };
        }
    }
}
