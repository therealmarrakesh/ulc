use std::{collections::HashSet, fmt};

#[derive(Clone, Debug)]
pub enum Expr {
    Variable { name: String },
    Abstraction { parameter: String, body: Box<Expr> },
    Application { left: Box<Expr>, right: Box<Expr> },
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
                    body.rename(old, new);
                }
            }
            Expr::Application { left, right } => {
                left.rename(old, new);
                right.rename(old, new);
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
