use crate::expr::Expr;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    bindings: HashMap<String, Rc<Expr>>,
}

impl Environment {
    pub fn new() -> Self {
        let bindings = HashMap::new();
        Environment { bindings }
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.bindings.get(name).map(|rc| (**rc).clone())
    }

    pub fn insert(&mut self, name: String, expr: Expr) {
        self.bindings.insert(name, Rc::new(expr));
    }
}
