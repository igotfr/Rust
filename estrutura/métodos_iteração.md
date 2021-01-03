```rust
fn main() {
  let array: [u8; 4] = [1, 2, 3, 4];

  let mapa: Vec<u8> = array.iter().map(|x: &u8| -> u8 { x * 2 }).collect::<Vec<u8>>(); // [2, 4, 6, 8]

  let filtro: Vec<u8> = array.iter().cloned().filter(|&x: &u8| -> bool { x > 2 }).collect::<Vec<u8>>(); // [3, 4]

  let encontrar: &u8 = array.iter().find(|&&x: &&u8| -> bool { x > 2 }).unwrap(); // 3

  let is_todos: bool = array.iter().all(|&x: &u8| -> bool { x > 2 }); // false

  let is_qualqer: bool = array.iter().any(|&x: &u8| -> bool { x > 2 }); // true
}
```
