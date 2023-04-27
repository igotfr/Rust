trait Int {}
impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for u128 {}
impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}

fn f(y: impl Int + std::fmt::Debug) {
  println!("{y:?}")
}

fn main() {
  f(56454454);
  f(-5)
}
