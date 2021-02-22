```rust
let arr: [u8] = u8[1, 2, 3, 4, 5, 6]; // vector Vec<u8>

let arr: [u8; 3] = <u8; 3>[1, 2, 3]; // exatamente 3 elementos, nem mais nem menos

let arr: [u8; 2..] = <u8; 2..>[1, 2, 3]; // mínimo 2 elementos

let arr: [u8; ..=4] = <u8; ..=4>[1, 2, 3, 4]; // máximo 4 elementos

let arr: [u8; 2..=4] = <u8; 2..=4>[1, 2, 3]; // mínimo 2 elementos, máximo 4 elementos
```
