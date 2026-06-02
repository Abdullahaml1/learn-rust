use std::{collections::HashMap, os::linux::net::SocketAddrExt};
fn main() {
    // The key and valute types has to the smae (all the kyes are the same and all the values of
    // the samte type
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Ahmed"), 9);
    scores.insert(String::from("Mohammed"), 3);

    // Acessing the valutes for a hash map
    let name = String::from("Ahmed");
    // The get method returns Option<&vec>
    let name_score = scores.get(&name).copied().unwrap_or(0);
    println!("The score is: {}", name_score);
    let name_score = &scores[&name];
    println!("The score is: {}", name_score);

    // Lopping over the hashmap
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // Overwriting a valute
    let mut scores = HashMap::new();
    scores.insert(String::from("Ahmed"), 9);
    scores.insert(String::from("Ahmed"), 1);
    println!("{:?}", scores);

    // Insert a key if not exists
    let mut scores = HashMap::new();
    scores.insert(String::from("Ahmed"), 9);
    let entry = scores.entry(String::from("Ahmed"));
    println!("{:?}", entry);
    match entry {
        std::collections::hash_map::Entry::Occupied(x) => println!("{:?}", x),
        std::collections::hash_map::Entry::Vacant(_) => todo!(),
    }

    scores.entry(String::from("Ahmed")).or_insert(-1);
    scores.entry(String::from("Mohammed")).or_insert(10);
    println!("{scores:?}");

    // updating a valulte based on old value
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Ahmed"), 9);
    scores.insert(String::from("Mohammed"), 3);
    for (k, v) in &mut scores {
        *v += 10;
    }
    println!("{:?}", scores);
    let x = scores.get_mut(&String::from("Ahmed")); // getting only the value
    if let Some(x_ref) = x {
        *x_ref = 33;
    }
    println!("{:?}", scores);
}
