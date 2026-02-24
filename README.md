# ulc

Untyped lambda calculus interpreter written in Rust. Implements normal-order reduction (leftmost-outermost redex first) with deferred substitution via an environment.

## REPL

cargo run

Toggle trace mode with :trace to see each reduction step.

> (λm. λn. λf. λx. m f (n f x)) (λf. λx. f (f x)) (λf. λx. f (f (f x)))
    -> λf. λx. f (f (f (f (f x))))
