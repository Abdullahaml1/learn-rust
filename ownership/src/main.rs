// ---------------------------------------------------------------
//Summary:
// We have wither: borrwoing (referencing) and ownership (moving)
//                     |
//             |-----------------|
//      Mutable Borrow       Imutable Borrow
//At any given time, you can have either one mutable reference or any number of immutable references.
//References must always be valid.
//
// ---------------------------------------------------------------
// fn main() {
//     let s = String::from("hello"); // s comes into scope
//
//     takes_ownership(s); // s's value moves into the function...
//     // ... and so is no longer valid here
//
//     // takes_ownership(s); // getting an error
//
//     let x = 5; // x comes into scope
//
//     makes_copy(x); // Because i32 implements the Copy trait,
//     // x does NOT move into the function,
//     // so it's okay to use x afterward.
// } // Here, x goes out of scope, then s. However, because s's value was moved,
// // nothing special happens.
//
// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.
//
// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.
//

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership moves its return
//     // value into s1
//
//     let s2 = String::from("hello"); // s2 comes into scope
//
//     let s3 = takes_and_gives_back(s2); // s2 is moved into
//     // takes_and_gives_back, which also
//     // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// // happens. s1 goes out of scope and is dropped.
//
// fn gives_ownership() -> String {
//     // gives_ownership will move its
//     // return value into the function
//     // that calls it
//
//     let some_string = String::from("yours"); // some_string comes into scope
//
//     some_string // some_string is returned and
//     // moves out to the calling
//     // function
// }
//
// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope
//
//     a_string // a_string is returned and moves out to the calling function
// }

//----------------------------------------------------------------------
// Borrwoing (Refrencing)
//----------------------------------------------------------------------
// fn main() {
//     let str = String::from("Hello");
//
//     // claaing the fuction without taking onwership
//     let len = calc_length(&str);
//     println!("The length of string is: {len}")
// }
//
// // This means borrowing
// fn calc_length(str: &String) -> usize {
//     str.len()
// }

//-----------------------------------------------------------------------
// Slicing
//-----------------------------------------------------------------------
fn main() {
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..];

    println!("w1= {hello}, w2= {world}");

    //-----------------------------------------------------------------------
    // Borrwing extening example NLL (Non Lexical Lifetime) The borrow is not activatge unless it
    // is actually used
    //-----------------------------------------------------------------------
    // Logical Error Case
    let mut x = 3;
    let x_ref = &mut x;
    // we can not print x as printing will borrow the reference to x which is already borrowed in
    // the let x_ref = &mut x; line
    // println!("x is `{}`, x_ref is `{}`", x, x_ref);
    // x = 9999; // ❌ BUG: ERROR: x is still borrowed
    *x_ref = 100; // x_ref is used here – the borrow is alive

    // Unlogical Case: we can set the value to x as normal
    let mut x = 3;
    let x_ref = &mut x;
    // we can not print x as printing will borrow the reference to x which is already borrowed in
    // the let x_ref = &mut x; line
    // println!("x is `{}`, x_ref is `{}`", x, x_ref);
    x = 9999; // ✅ works! No error
    // x_ref is never used again

    // --------------------------------------------------------------
    // Rust uses copying instead of moving for simple types and with types that implements copy and
    // clone traits
    // --------------------------------------------------------------

    //here we can use x as long as we can without moving it
    fn print_x(x: u32) {
        ()
    }
    let x = 3;
    print_x(x);
    print_x(x);
    print_x(x);
    print_x(x);

    // here the vlaue for data will be moved
    struct Data {
        x: u32,
    }
    fn print_data(x: Data) {
        ()
    }
    let x = Data { x: 3 };
    print_data(x);
    // print_data(x); // BUG: value has moved we can not compile

    // here the value is copied so no moving is happening
    #[derive(Copy, Clone)]
    struct DataCloned {
        x: u32,
    }
    fn print_data_cloned(x: DataCloned) {
        ()
    }
    let x = DataCloned { x: 3 };
    print_data_cloned(x);
    print_data_cloned(x);
    print_data_cloned(x);
    print_data_cloned(x);
}
