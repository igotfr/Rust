```rust
Vec
  - * inserts(0:'a', 1:'b', 2:'c') VARARG
  - * splices([ 1..2, &[3, 4] ], [ 4..5, &[6, 7] ]) VARARG

  - * removes(2, 5, 9) VARARG
  - * drains(0..2, 5..7) VARARG

  - * remove_items(&1, &8) VARARG
  - * remove_item_nth(&1, 2)
  - * remove_items_nth([&1, 2], [&7, 4]) VARARG

___________________________

VecDeque
  - prepend() exist in LinkedList
  - * inserts(0:'a', 1:'b', 2:'c') VARARG
  - splice(1..2, &[3, 4]) exist in Vec
  - * splices([ 1..2, &[3, 4] ], [ 4..5, &[6, 7] ]) VARARG

  - * removes(2, 5, 9) VARARG
  - * drains(0..2, 5..7) VARARG

  - remove_item(&1) exist in Vec
  - * remove_items(&1, &8) VARARG
  - * remove_item_nth(&1, 2)
  - * remove_items_nth([&1, 2], [&7, 4]) VARARG
```
