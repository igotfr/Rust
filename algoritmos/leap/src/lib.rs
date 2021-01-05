pub fn is_leap_year(year: u64) -> bool {
  //unimplemented!("true if {} is a leap year", year)
  let year_div = |n: u64| -> bool { year % n == 0 };
  year_div(4) && !year_div(100) || year_div(400)
}
