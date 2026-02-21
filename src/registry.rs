use crate::expr::Expr;
use std::collections::HashSet;

pub struct Registry {
    names: HashSet<String>,
}

impl Registry {
    pub fn new(expr: &Expr) -> Self {
        let names = expr.collect_names();
        Registry { names }
    }

    pub fn fresh_name(&mut self, base: &str) -> String {
        let mut name = base.to_string();
        let mut i = 1;
        while self.names.contains(&name) {
            name = format!("{}{}", base, i);
            i += 1;
        }
        self.names.insert(name.clone());
        name
    }
}
