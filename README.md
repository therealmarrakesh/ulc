# ulc

Untyped lambda calculus interpreter written in Rust. Implements normal-order reduction (leftmost-outermost redex first) with deferred substitution via an environment. Performs alpha conversion to avoid variable capture during reduction.

## Syntax

- Lambda: `λx. body` or `\x. body`
- Application: `f x` (left-associative)
- Grouping: `(expr)`
- Variable names: start with a letter, followed by letters, digits, or underscores

## REPL

`cargo run`

Toggle trace mode with `:trace` to see each reduction step.

Adding Church numerals 2 and 3:

```
(λm. λn. λf. λx. m f (n f x)) (λf. λx. f (f x)) (λf. λx. f (f (f x)))
```

Reduces to Church numeral 5:

```
-> (λn. λf. λx. m1 f (n f x)) (λf. λx. f (f (f x)))
-> λf. λx. m1 f (n1 f x)
-> λf. λx. (λf. λx. f (f x)) f (n1 f x)
-> λf. λx. (λx. f1 (f1 x)) (n1 f x)
-> λf. λx. f1 (f1 x1)
-> λf. λx. f (f1 x1)
-> λf. λx. f (f x1)
-> λf. λx. f (f (n1 f x))
-> λf. λx. f (f ((λf. λx. f (f (f x))) f x))
-> λf. λx. f (f ((λx. f2 (f2 (f2 x))) x))
-> λf. λx. f (f (f2 (f2 (f2 x2))))
-> λf. λx. f (f (f (f2 (f2 x2))))
-> λf. λx. f (f (f (f (f2 x2))))
-> λf. λx. f (f (f (f (f x2))))
-> λf. λx. f (f (f (f (f x))))
```
