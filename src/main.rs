pub mod environment;
pub mod evaluator;
pub mod expr;
pub mod lexer;
pub mod parser;
pub mod registry;
pub mod token;

use environment::Environment;
use evaluator::Evaluator;
use expr::Expr;
use lexer::Lexer;
use parser::Parser;
use registry::Registry;
use std::io::{Write, stdin, stdout};

fn main() {
    repl();
}

pub fn repl() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit" {
                    break;
                }
                if input.trim().is_empty() {
                    continue;
                }
                eval(&input);
            }
            None | Some(Err(_)) => break,
        }
    }
}

fn eval(input: &str) {
    let result = evaluate(input);
    println!("  {}\n", result);
}

fn evaluate(input: &str) -> Expr {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let expr = parser.parse_expression();
    let env = Environment::new();
    let registry = Registry::new(&expr);
    let evaluator = Evaluator::new(expr, env, registry);
    evaluator.reduce()
}
