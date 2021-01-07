pub fn verse(n: u32) -> String {
  match n {
    0  => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
    1  => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
    _  => {
      let s: &str = if n > 2 { "s" } else { "" };
      format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n , n - 1, s)
    }
  }
}

pub fn sing(start: u32, end: u32) -> String {
  //unimplemented!("sing verses {} to {}, inclusive", start, end)
  (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
