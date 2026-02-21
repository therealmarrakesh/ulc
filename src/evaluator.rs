use crate::environment::Environment;
use crate::expr::Expr;
use crate::registry::Registry;

pub struct Evaluator {
    expr: Expr,
    env: Environment,
    registry: Registry,
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
                if let Expr::Abstraction {
                    parameter,
                    mut body,
                } = *left
                {
                    //generate fresh name and store the argument in the environment
                    let fresh = registry.fresh_name(&parameter);
                    env.insert(fresh.clone(), *right);

                    // rename the parameter to the fresh name in the body
                    body.rename(&parameter, &fresh);

                    //write the modified body back into expr
                    *expr = *body;
                    return true;
                }

                // not a redex - put the Application back, but try to reduce inside it
                *expr = Expr::Application { left, right };
            }
        }

        // Normal order reduction (leftmost-outermost)
        match expr {
            Expr::Application { left, right } => {
                Self::step(left, env, registry) || Self::step(right, env, registry)
            }
            Expr::Abstraction { body, .. } => Self::step(body, env, registry),
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
