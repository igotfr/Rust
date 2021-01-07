use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  //unimplemented!("Write a function to reverse {}", input);
  //input.chars().rev().collect::<String>()
  input.graphemes(true).rev().collect::<String>()
}
