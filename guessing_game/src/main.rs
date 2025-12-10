use rand::Rng; // we are only using `Rng` trait of `rand` crate
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //start..=end

    println!("The secrect number is: `{secret_number}`");

    // infnite loop
    loop {
        println!("Pleae input your guess");
        // Variables and refrences (&) are imutables
        // to make them mutable we use `mut`
        // `new is an acsociated function to the `String` type
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // return an `Enum` of type `Resutlt` which has method expect to
            // raise an error if the Reusrt is retunred as Result::Err or Result:Ok
            .expect("Failed to read line");

        // Shadowing we use the variable name but with a differrent value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You have to input integer not string");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
