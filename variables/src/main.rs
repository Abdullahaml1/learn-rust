const HAMO: u32 = 3 * 50;

// errror varibles have to be in functions
// let x = 3;
fn main() {
    println!("Hello, world!");
    println!("How are you {HAMO}");

    // Shadowing !!!!!
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner Loop x is `{x}`")
    }
    println!("Outer Loop x is `{x}`");

    // Imutable variables can change its type
    let spaces = "    "; // strinInner
    let spaces = spaces.len(); // interger
    //
    //
    // Mutable variables can not change its tpe
    // let mut spaces = "    "; // strinInner
    // spaces = spaces.len(); // interger
}
