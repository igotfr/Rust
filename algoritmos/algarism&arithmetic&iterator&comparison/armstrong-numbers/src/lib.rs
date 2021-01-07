pub fn is_armstrong_number(num: u32) -> bool {
  //unimplemented!("true if {} is an armstrong number", num)
  let num_to_str: String = num.to_string();
  let num_to_str_len: u32 = num_to_str.len() as u32;

  num == num_to_str.chars().map(|c: char| -> u32 { c.to_digit(10).unwrap().pow(num_to_str_len) }).sum()
}
