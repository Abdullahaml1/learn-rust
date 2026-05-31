// Enumn share most feature with structs

// ----------------------------------------------------------
// Simple enum
// ----------------------------------------------------------
#[derive(Debug)]
enum Message {
    Sucess,
    Fail,
}

// ----------------------------------------------------------
// Enum with Type
// ----------------------------------------------------------
#[derive(Debug)]
enum IPVersion {
    V4,
    V6,
}
#[derive(Debug)]
struct Metadata {
    name: String,
    vender: String,
    header: String,
}
#[derive(Debug)]
enum CompoundMessage {
    Version(IPVersion),
    Valid(bool),
    Metadata(Metadata),
}

impl CompoundMessage {
    // Any instance from the enum will inherate this method
    fn quack(&self) {
        println!("Quak");
    }
}

// ----------------------------------------------------------
// Matching Block
// ----------------------------------------------------------
#[derive(Debug)]
enum City {
    Cairo,
    Alex,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(City),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("The coin is Dime");
            10
        }
        Coin::Quarter(city) => match city {
            City::Cairo => 25,
            City::Alex => 50,
        },
    }
}

// ----------------------------------------------------------
// Using match with Option<T>
// ----------------------------------------------------------
fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    // ----------------------------------------------------------
    // Simple enum
    // ----------------------------------------------------------
    let m1 = Message::Sucess;
    println!("The messages is: {:?}", m1);
    let version = CompoundMessage::Version(IPVersion::V4);
    println!("Version: {:?}", version);
    version.quack();

    // ----------------------------------------------------------
    // Option Enum
    // ----------------------------------------------------------
    // Rust has no Null by default but has `Option` Enum that is defined as:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // Option Enum has useful methods see: https://doc.rust-lang.org/stable/std/option/enum.Option.html
    let x: Option<u32> = Some(11);
    let y: u32 = 3;
    let z: Option<u32> = None;
    // println!("x + y = {}", x + y); // error x and y are nto the same type
    if x.is_some() {
        println!("x + y = {}", x.unwrap() + y);
    }
    if !z.is_none() {
        println!("z + y = {}", z.unwrap() + y);
    } else {
        println!("We can not add z + y because z is `None`");
    }

    // ----------------------------------------------------------
    // Matching Block
    // ----------------------------------------------------------
    let coin = Coin::Dime;
    let val = value_in_cents(&coin);
    println!("val is: {}", val);
    println!("Val is: {:?}", coin);

    let alex_coin = Coin::Quarter(City::Alex);
    let alex_val = value_in_cents(&alex_coin);
    println!("val is: {}", alex_val);
    println!("Val is: {:?}", alex_coin);
    // ----------------------------------------------------------
    // Using match with Option<T>
    // ----------------------------------------------------------
    let x: Option<u32> = Some(10);
    println!("x + 1 is: {:?}", plus_one(x));

    // ----------------------------------------------------------
    // Catching all match patterns
    // ----------------------------------------------------------
    let x: u32 = 10;
    match x {
        3 => println!("3"),
        4 => println!("4"),
        other => println!("This is other than what we know: `{other}`"),
    }
    let x: u32 = 10;
    match x {
        3 => println!("3"),
        4 => println!("4"),
        _ => (), // nothing
    }
    // ----------------------------------------------------------
    // if let statement
    // ----------------------------------------------------------
    let config_max: Option<u32> = Option::Some(100);
    match config_max {
        Option::Some(max) => print!("The max is: {max}"),
        Option::None => (),
    }
    // Or we can write it as:
    let config_max = Some(100);
    match config_max {
        Some(max) => print!("The max is: {max}\n"),
        _ => (),
    }

    // To avoid writing the `_ +>()` becuase matches are exhustive we use a if let keyword
    // ignoring the else
    let config_max = Some(100);
    if let Some(max) = config_max {
        print!("The config max is: {max}\n");
    }

    let mut count = 0;
    let coin = Coin::Quarter(City::Alex);
    if let Coin::Quarter(state) = coin {
        print!("This is a quarter of state: {state:?}");
    } else {
        count += 1;
    }
}
