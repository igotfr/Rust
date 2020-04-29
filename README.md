VecDeque
  - prepend() exist in 
  - * insert_sev[eral](0:'a', 1:'b', 2:'c') VARARG
  - splice(1..2, &[3, 4]) exist in Vec
  - * splice_sev[eral]([ 1..2, &[3, 4] ], [ 4..5, &[6, 7] ]) VARARG

  - * remove_sev[eral](2, 5, 9) VARARG
  - * drain_sev[eral](0..2, 5..7) VARARG

  - remove_item(&1) exist in Vec
  - * remove_items(&1, &8) VARARG
  - * remove_item_nth(&1, 2)
  - * remove_items_nth([&1, 2], [&7, 4]) VARARG
