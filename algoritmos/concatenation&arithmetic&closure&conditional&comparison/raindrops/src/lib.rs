pub fn raindrops(n: u32) -> String {
  //unimplemented!("what sound does Raindrop #{} make?", n)
  let n_div = |x: u32| -> bool { n % x == 0 };

  //if !n_div(3) && !n_div(3) && !n_div(3) { return n.to_string() }

  let mut result: String = String::new();

  if n_div(3) { result.push_str("Pling") }
  if n_div(5) { result.push_str("Plang") }
  if n_div(7) { result.push_str("Plong") }
  if result == "" { return n.to_string() }

  result
}
