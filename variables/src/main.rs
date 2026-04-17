const HAMO: u32 = 3 * 50;

// errror varibles have to be in functions
// let x = 3;
fn main() {
    println!("Hello, world!");
    println!("How are you {HAMO}");

    // Shadowing !!!!!
    // variables are imutable by default
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner Loop x is `{x}`")
    }
    println!("Outer Loop x is `{x}`");

    let mut x = 1_333; // this is the same as 1333 `_` for readablity
    let y = 0xff; // hexal
    let y = 0o77; // octal
    let y = 0b111_000; //binary `_` for readablity
    let y_byte: u8 = b'A'; // ascci

    // Imutable variables can change its type
    let spaces = "    "; // strinInner
    let spaces = spaces.len(); // interger
    //
    //
    // Mutable variables can not change its tpe
    // let mut spaces = "    "; // strinInner
    // spaces = spaces.len(); // interger
}
