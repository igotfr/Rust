// According to https://en.wikipedia.org/wiki/Square_pyramidal_number
pub fn sum_of_squares(n: u64) -> u64 {
  // n * (n + 1) * (2 * n + 1) / 6
  // (n * n * (n + 1) * (n + 1)) / 4
}

// According to https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
pub fn square_of_sum(n: u64) -> u64 {
  (n * (n + 1) / 2).pow(2)
}

pub fn difference(number: u64) -> u64 {
  square_of_sum(number) - sum_of_squares(number)
}
