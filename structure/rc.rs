// https://doc.rust-lang.org/book/ch15-04-rc.html

use std::rc::Rc;

#[derive(Debug)]
struct Truck;

fn main() {
    let a = Rc::new(Truck);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("{a:?} {b:?}");
    println!("reference count: {}", Rc::strong_count(&a));

    std::mem::drop(b);
    println!("{a:?} {c:?}");
    println!("reference count: {}", Rc::strong_count(&a))
}
