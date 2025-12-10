fn main() {
    println!("Hello, world!");
    let mut x = 1_333; // this is the same as 1333 `_` for readablity
    let y = 0xff; // hexal
    let y = 0o77; // octal
    let y = 0b111_000; //binary `_` for readablity
    let y_byte: u8 = b'A'; // ascci

    // if happend integer overflow the compiler will raise a `panic` in debug mode
    // and will do two's complement in the release mode
    //
    // We can avoid overflow by the following:
    //
    //
    // size is architecture depednent u64 for 64 and u32 for 32
    let s: usize = 3;
    let s: isize = 3; // singed
    //
    //Floats
    let x = 3.5; // f64 rust default
    let x: f32 = 3.2;
    // let x: f16 = 3.2; //unstable
    //
    //
    // Boolean are 1 byte in size
    let t = true;
    let t: bool = false;

    // character
    let c = 'A'; // single qoutes are charctes, double are string
    let c: char = 'A';

    // tuples
    let t: (u32, f64, u8) = (3, 3., 3);
    let (x, y, z) = t;
    // or
    let x = t.0;
    let y = t.1;
    let z = t.2;

    // Arrays lives in the stack (faster)
    let arr = [1, 2, 3, 4, 5];
    let arr: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
    // of strings
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5]; // contain 5 3s [3, 3, ,3, 3, 3]
    let first = a[0];
    let second = a[1];
}
