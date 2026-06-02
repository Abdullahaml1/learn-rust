use std::char;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Creating new string
    let s = String::new();

    // Creating a Strign form the builtin string type
    let s = "السلام عليكم ورحمة الله وبركاته";
    let s = s.to_string();

    // pusing to a string
    let mut s = String::from("Hello");
    s.push_str(" World!");
    println!("{}", s);

    let mut s = String::from("Hello");
    let world = " World";
    s.push_str(world); // it does not take the ownership for the world string
    println!("{}", s);
    println!("{}", world);

    // Adding single character to a string
    let mut s = String::from("Hello");
    s.push('!');
    println!("{}", s);

    // Concatenating two strings
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s = s1 + &s2;
    println!("{}", s);
    // println!("{}", s1); // BUG: s1 has moved to s1 so we can not use it agin
    println!("{}", s2); // NOTE: Normal code as s2 is just borrwoed but not moved
    //
    // Concatenating more than a string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // Indexing into a string
    let s = String::from("السَّلامُ علَيْكُمْ");
    // let h = &s[0]; //BUG: to index a string either use .chars().nth() or .bytes().nth()
    if let Some(t) = s.chars().nth(0) {
        println!("{}", t);
    }
    if let Some(t) = s.bytes().nth(0) {
        println!("{}", t);
    }
    println!("\n\n\n");
    println!("Chars is `{:?}`", s.chars());
    println!("Number of Chars is `{}`", s.chars().count());
    println!("Bytes is `{:?}`", s.bytes());
    println!("Number of bytes is `{}`", s.bytes().len());
    let mut graphems_vec: Vec<String> = Vec::new();
    for g in s.graphemes(true) {
        graphems_vec.push(g.to_string());
    }
    println!("Graphemes is `{:?}`", graphems_vec);
    for g in graphems_vec {
        println!("{}\n", g);
    }
    println!("Number of graphemes is `{}`", s.graphemes(true).count());

    // Slicing a strin accourding to it bytes
    let s = String::from("السَّلامُ علَيْكُمْ");
    println!("{}", &s[0..4]); // works normal
    // println!("{}", &s[0..1]); // BUG: Panics as we are slicing the string on its bytes

    // Iterating overa string (chars)
    let s = String::from("السَّلامُ علَيْكُمْ");
    for c in s.chars() {
        println!("{}", c);
    }

    // Iterating overa string (bytes)
    let s = String::from("السَّلامُ علَيْكُمْ");
    for c in s.bytes() {
        println!("{}", c);
    }
    // str type when initilized it reserved a data space in the binary (written to the binary) so
    // we can make something like this (refrence a varialbe that lived locally in the stack):
    let s = "Hello".to_string() + &" World";
    //                            ^
}
