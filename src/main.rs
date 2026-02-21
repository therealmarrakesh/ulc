pub mod environment;
pub mod evaluator;
pub mod expr;
pub mod lexer;
pub mod parser;
pub mod registry;
pub mod repl;
pub mod token;

fn main() {
    repl::repl();
}
