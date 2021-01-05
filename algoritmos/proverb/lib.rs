pub fn build_proverb(list: &[&str]) -> String {
  //unimplemented!("build a proverb from this list of items: {:?}", list)
  (0..list.len()).map(|i: usize| -> String {
    if i == list.len() - 1 { format!("And all for the want of a {}.", list[0]) }
    else { format!("For want of a {} the {} was lost.", list[i], list[i + 1]) }
  }).collect::<Vec<String>>().join("\n")
}
