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

    // ------------------------------------------------------
    // Box<T> implements Deref so we can do `*box` or `&box`
    // ------------------------------------------------------
    let mut x_ref = Box::new(3);
    println!("*x = {}", *x_ref);
    *x_ref = 10;
    println!("*x = {}", *x_ref);

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            MyBox(x)
        }
    }
    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> std::ops::DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    let mut my_x_ref = MyBox::new(3);
    println!("*x = {}", *my_x_ref); // NOTE: this is Deref
    *my_x_ref = 10; // NOTE: DerefMute
    println!("*x = {}", *my_x_ref);

    // ---------------------------------------------------------------
    // Deref Coersion (الإكراه) return another typs when calling deref
    // For example return a str when calling & or *(a String)
    // ---------------------------------------------------------------
    let s = String::from("Hello");
    println!(
        "String implements Deref and returns a &str when called with &String\ns[..] is str and &s[..] is a ref to str\nSo &s == &s[..] is {}",
        (&s == &s[..]) as bool
    );
    let s = Box::new(String::from("Hello"));
    fn hello_fn(x: &str) {
        println!("{}", x);
    }
    hello_fn(&s);
    hello_fn(&(*s)[..]);

    // -----------------------------------------------------------------
    // Drop taait is called automaticlly after the variable gone out of sckope for by calling
    // `drop` directly. we can consider this trait as a callback function called when drop the
    // variable
    // -----------------------------------------------------------------
    struct HamoStruct {}
    impl HamoStruct {
        fn new() -> Self {
            Self {}
        }
    }
    impl Drop for HamoStruct {
        fn drop(&mut self) {
            println!("Droping HamoStruct!");
        }
    }
    let h = HamoStruct::new();
}
