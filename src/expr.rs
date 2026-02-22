use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Clone, Debug)]
pub enum Expr {
    Variable { name: String },
    Abstraction { parameter: String, body: Rc<Expr> },
    Application { left: Rc<Expr>, right: Rc<Expr> },
}

impl Expr {
    pub fn rename(&mut self, old: &str, new: &str) {
        match self {
            Expr::Variable { name } => {
                if name == old {
                    *name = new.to_string();
                }
            }
            Expr::Abstraction { parameter, body } => {
                if parameter != old {
                    Rc::make_mut(body).rename(old, new);
                }
            }
            Expr::Application { left, right } => {
                Rc::make_mut(left).rename(old, new);
                Rc::make_mut(right).rename(old, new);
            }
        }
    }

    pub fn collect_names(&self) -> HashSet<String> {
        match self {
            Expr::Variable { name } => {
                let mut set = HashSet::new();
                set.insert(name.clone());
                set
            }
            Expr::Abstraction { parameter, body } => {
                let mut set = body.collect_names();
                set.insert(parameter.clone());
                set
            }
            Expr::Application { left, right } => {
                let mut set = left.collect_names();
                set.extend(right.collect_names());
                set
            }
        }
    }

    pub fn contains_free(&self, target: &str) -> bool {
        match self {
            Expr::Variable { name } => name == target,
            Expr::Abstraction { parameter, body } => {
                parameter != target && body.contains_free(target)
            }
            Expr::Application { left, right } => {
                left.contains_free(target) || right.contains_free(target)
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Variable { name } => write!(format, "{}", name),
            Expr::Abstraction { parameter, body } => write!(format, "λ{}. {}", parameter, body),
            Expr::Application { left, right } => {
                match left.as_ref() {
                    Expr::Abstraction { .. } => write!(format, "({})", left)?,
                    _ => write!(format, "{}", left)?,
                }
                write!(format, " ")?;
                match right.as_ref() {
                    Expr::Application { .. } | Expr::Abstraction { .. } => {
                        write!(format, "({})", right)
                    }
                    _ => write!(format, "{}", right),
                }
            }
        }
    }
}
