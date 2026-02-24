use crate::environment::Environment;
use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::registry::Registry;
use std::io::{Write, stdin, stdout};

enum ReplMode {
    Normal,
    Trace,
}

pub fn repl() {
    let mut mode = ReplMode::Normal;
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                let trimmed = input.trim();
                match trimmed {
                    "exit" => break,
                    ":trace" => {
                        mode = match mode {
                            ReplMode::Normal => {
                                println!("  trace on\n");
                                ReplMode::Trace
                            }
                            ReplMode::Trace => {
                                println!("  trace off\n");
                                ReplMode::Normal
                            }
                        };
                        continue;
                    }
                    "" => continue,
                    _ => evaluate(trimmed, &mode),
                }
            }
            None | Some(Err(_)) => break,
        }
    }
}

fn evaluate(input: &str, mode: &ReplMode) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(expr) => {
            let initial = expr.clone();
            let env = Environment::new();
            let registry = Registry::new(&expr);
            let evaluator = Evaluator::new(expr, env, registry);

            match mode {
                ReplMode::Normal => {
                    let result = evaluator.last().unwrap_or(initial);
                    println!("  -> {}\n", result);
                }
                ReplMode::Trace => {
                    let mut last_step = None;
                    for step in evaluator {
                        if let Some(prev) = last_step {
                            println!("  -> {}", prev);
                        }
                        last_step = Some(step);
                    }
                    match last_step {
                        Some(final_expr) => println!("  -> {}\n", final_expr),
                        None => println!("  -> {}\n", initial),
                    }
                }
            }
        }
        Err(e) => {
            println!("  Parse Error: {}\n", e);
        }
    }
}
