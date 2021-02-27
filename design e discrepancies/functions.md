https://github.com/rust-lang/rfcs/issues/323

## Default parameter value by position

```rust
fn foo(a: i8, b: f32 = 5.6, c: Option<bool> = None) {}

foo(7);
foo(7, 6.9);
foo(7, 6.9, Some(true));
```
