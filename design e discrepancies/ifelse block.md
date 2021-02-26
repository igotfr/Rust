https://github.com/rust-lang/rfcs/issues/3024

and if if-else were blocks similar to match

```rust
let a = 2;

ifelse {
  a > 3 => println!("true"),
  5 < 1 {
    // code
  },
  _ => println!("no one")
}
```

instead of:

```rust
let a = 2;

if a > 3 {
  println!("true");
} else if 5 < 1 {
   // code
} else {
  println!("no one");
}
```
