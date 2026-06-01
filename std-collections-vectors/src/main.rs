use std::any::Any;

fn main() {
    println!("Hello, world!");
    // ---------------------------------------------------
    // Vectors
    // ---------------------------------------------------
    let vec: Vec<i32> = Vec::new();
    println!("vec: {:?}", vec);
    let mut vec = vec![1, 2, 3];
    println!("vec: {:?}", vec);
    println!("Vector type is: {}", std::any::type_name_of_val(&vec));

    // pushing to the vector
    vec.push(4);
    vec.push(5);
    println!("vec: {:?}", vec);

    // Reading elemetns form a vector
    let third: &i32 = &vec[3];
    println!("third={}", third);

    // get will return Option
    let third: Option<&i32> = vec.get(3);
    match third {
        Some(third) => println!("thrid={}", third),
        None => println!("No value found"),
    }

    // borrwing an element from the vector is the same as borrowing the vector it self
    let mut vec = vec![1, 2, 3, 4, 5];
    let first = &vec[0];
    // vec.push(6); // will not compile becase an element value is copied
    println!(
        "Using the value to avoid NLL (Non-Lexical Lifetime) {}",
        first
    );

    // Iterating over a vector
    let vec = vec![0, 1, 2];
    for elm in &vec {
        println!("elm is {}", elm);
    }
    for i in 0..vec.len() {
        println!("vec[{}]={}", i, vec[i]);
    }

    // Iterating over mutable vector
    let mut vec = vec![0, 1, 2, 3];
    for x in &mut vec {
        *x += 50;
    }
    println!("{:?}", vec);

    // Storing an Enum
    enum SpreadSheet {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let row = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Float(3.1),
        SpreadSheet::Text(String::from("Hello")),
    ];
}
