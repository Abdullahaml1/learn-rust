use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
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

    // ----------------------------------------------------------------------------
    // `Rc` (Reference Counting) is a smart pointer to the heap which can hold multpir refs with
    // mour than one variable and it will be droped if the all references are droped
    // NOTE: it can not be used across threads use `Arc`
    // ----------------------------------------------------------------------------
    let a = Rc::new(ListRc::Cons(3, Rc::new(ListRc::Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::new(ListRc::Cons(4, Rc::clone(&a)));
    println!("count after b ref = {}", Rc::strong_count(&a));
    let c = Rc::new(ListRc::Cons(6, Rc::clone(&a)));
    println!("count after c ref = {}", Rc::strong_count(&a));
    println!("{:?}\n{:?}\n{:?}", a, b, c);

    // -----------------------------------------------------------------------------------------
    // Intrior Mutability look at the `lib.rs`
    // -----------------------------------------------------------------------------------------

    // -----------------------------------------------------------------------------------------
    // Alllowing multiple owners (more than one mutable borrow) for the same data. Breaking rust
    // borrow rules which is: we can not have more thean ONE mutable borrow  (owner) of the same
    // data.
    // -----------------------------------------------------------------------------------------
    #[derive(Debug)]
    struct Hamo {
        x: u32,
    }
    let ptr = Rc::new(RefCell::new(Hamo { x: 10 }));
    println!("ptr: {:?}", ptr);

    let ptra = Rc::clone(&ptr);
    println!("ptra: {:?}", ptra);

    let ptrb = Rc::clone(&ptr);
    println!("ptrb: {:?}", ptrb);

    ptr.borrow_mut().x = 9;
    println!("ptr: {:?}", ptr);
    println!("ptra: {:?}", ptra);
    println!("ptrb: {:?}", ptrb);
    ptra.borrow_mut().x = 3;
    println!("ptr: {:?}", ptr);
    println!("ptra: {:?}", ptra);
    println!("ptrb: {:?}", ptrb);
    ptrb.borrow_mut().x = 1;
    println!("ptr: {:?}", ptr);
    println!("ptra: {:?}", ptra);
    println!("ptrb: {:?}", ptrb);

    let mut hamo = Hamo { x: 10 };
    let ptr = &mut hamo;
    let ptra = &mut hamo;
    let ptrb = &mut hamo;

    // BUG: error we can have only one owner (mutable borrow) for the same data
    // without any usage of the pointers the code is not compling
    // println!("ptr: {:?}", ptr);
    // println!("ptra: {:?}", ptra);
    // println!("ptrb: {:?}", ptrb);

    // bypassing rust borrow rules
    #[derive(Debug)]
    struct HamoVec {
        x: Vec<u32>,
    }
    let ptr = Rc::new(RefCell::new(HamoVec { x: vec![] }));
    println!("ptr: {:?}", ptr);

    let ptra = Rc::clone(&ptr);
    println!("ptra: {:?}", ptra);

    let ptrb = Rc::clone(&ptr);
    println!("ptrb: {:?}", ptrb);

    ptr.borrow_mut().x.push(0);
    println!("ptr: {:?}", ptr);
    println!("ptra: {:?}", ptra);
    println!("ptrb: {:?}", ptrb);
    ptra.borrow_mut().x.push(1);
    println!("ptr: {:?}", ptr);
    println!("ptra: {:?}", ptra);
    println!("ptrb: {:?}", ptrb);
    ptrb.borrow_mut().x.push(2);
    println!("ptr: {:?}", ptr);
    println!("ptra: {:?}", ptra);
    println!("ptrb: {:?}", ptrb);
}
