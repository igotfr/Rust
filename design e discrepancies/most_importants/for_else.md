https://github.com/rust-lang/rfcs/issues/3361

```rust
fn main() {
  let result = for counter in 0..5 {
    if counter == 3 {
      break i
    }
  } else { -7 };

  println!("The result is {result}") // 3
}
```
```rust
fn main() {
  let result = for counter in 0..5 {
    if counter == 90 {
      break i
    }
  } else { -7 };

  println!("The result is {result}") // -7
}
```
#### Alternatives
https://rust-lang.github.io/rfcs/2046-label-break-value.html
