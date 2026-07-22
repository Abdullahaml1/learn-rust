#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    // -----------------------------------------
    // Box is heap applocated only with no fancy features
    // -----------------------------------------
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive Type:  linked list (Cons List) {:?} ", l);
}
