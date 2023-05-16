## Named parameter
```rust
fn foo(a: i8, nam b: f32, nam mut c: bool) {}

foo(7, b: 6.9, c: true);
foo(7, c: true, b: 6.9);
```

## Optional Named parameter
```rust
fn foo(a: i8, nam b: f32, opt mut c: bool) {}


foo(7, b: 6.9, true);
foo(7, b: 6.9, c: true);
foo(7, c: true, b: 6.9);
```

## Default parameter value by position
https://github.com/rust-lang/rfcs/issues/323

```rust
fn foo(a: i8, b: f32 = 5.6, c: Option<bool> = None) {}

foo(7);
foo(7, 6.9);
foo(7, 6.9, Some(true));
```
