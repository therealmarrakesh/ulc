use crate::environment::Environment;
use crate::expr::Expr;
use crate::registry::Registry;
use std::rc::Rc;

pub struct Evaluator {
    expr: Expr,
    env: Environment,
    registry: Registry,
}

impl Iterator for Evaluator {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        if Self::step(&mut self.expr, &mut self.env, &mut self.registry) {
            Some(self.expr.clone())
        } else {
            None
        }
    }
}

impl Evaluator {
    pub fn new(expr: Expr, env: Environment, registry: Registry) -> Self {
        Evaluator {
            expr,
            env,
            registry,
        }
    }

    pub fn reduce(mut self) -> Expr {
        loop {
            if !Self::step(&mut self.expr, &mut self.env, &mut self.registry) {
                return self.expr;
            }
        }
    }

    fn avoid_capture(body: &mut Expr, arg: &Expr, registry: &mut Registry) {
        match body {
            Expr::Abstraction { parameter, body } => {
                if arg.contains_free(parameter) {
                    let fresh = registry.fresh_name(parameter);
                    Rc::make_mut(body).rename(parameter, &fresh);
                    *parameter = fresh;
                }
                Self::avoid_capture(Rc::make_mut(body), arg, registry);
            }
            Expr::Application { left, right } => {
                Self::avoid_capture(Rc::make_mut(left), arg, registry);
                Self::avoid_capture(Rc::make_mut(right), arg, registry);
            }
            Expr::Variable { .. } => {}
        }
    }

    fn step(expr: &mut Expr, env: &mut Environment, registry: &mut Registry) -> bool {
        if let Expr::Application { .. } = expr {
            // take ownership of the Application node
            let owned = std::mem::replace(
                expr,
                Expr::Variable {
                    name: String::new(),
                },
            );

            if let Expr::Application { left, right } = owned {
                // peek without consuming — Rc can't move out with *left
                if let Expr::Abstraction { .. } = left.as_ref() {
                    let Expr::Abstraction {
                        parameter,
                        mut body,
                    } = Rc::unwrap_or_clone(left)
                    else {
                        unreachable!()
                    };

                    // alpha-rename inner binders that would capture free variables in the argument
                    Self::avoid_capture(Rc::make_mut(&mut body), right.as_ref(), registry);

                    // generate fresh name and store the argument in the environment
                    let fresh = registry.fresh_name(&parameter);
                    env.insert(fresh.clone(), right);

                    // rename the parameter to the fresh name in the body
                    Rc::make_mut(&mut body).rename(&parameter, &fresh);

                    // write the modified body back into expr
                    *expr = Rc::unwrap_or_clone(body);
                    return true;
                }

                // not a redex - put the Application back, but try to reduce inside it
                *expr = Expr::Application { left, right };
            }
        }

        // Normal order reduction (leftmost-outermost)
        match expr {
            Expr::Application { left, right } => {
                Self::step(Rc::make_mut(left), env, registry)
                    || Self::step(Rc::make_mut(right), env, registry)
            }
            Expr::Abstraction { body, .. } => Self::step(Rc::make_mut(body), env, registry),
            Expr::Variable { name } => {
                if let Some(term) = env.get(name) {
                    *expr = term;
                    true
                } else {
                    false
                }
            }
        }
    }
}
