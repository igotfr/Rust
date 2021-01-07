pub fn raindrops(n: u32) -> String {
  let drops: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

  let raindrop: String = drops.iter()
    .filter(|(factor, _): &&(u32, &str)| -> bool { n % factor == 0 })
    .map(|&(_, sound): &(u32, &str)| -> &str { sound })
    .collect::<String>();

  if raindrop == "" { return n.to_string() }

  raindrop
}
