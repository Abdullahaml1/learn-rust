use std::fs::{self, File};
use std::io::{self, ErrorKind, Read, Seek, Write};
use std::ops::RemAssign;

fn main() {
    // panic!("Error Here");
    // let v = vec![1, 2, 3];
    // let w = &v[99];
    // println!("{:?}", v);
    // println!("{:?}", w);
    /*
        Compiling error-handilng v0.1.0 (/home/abdullah/Documents/wakeb/learn-rust/error-handilng)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
         Running `target/debug/error-handilng`

    thread 'main' (161068) panicked at src/main.rs:4:15:
    index out of bounds: the len is 3 but the index is 99
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        */
    // To Enable for error traceback set the env varialble
    //RUST_BACKTRACE=1 or RUST_BACKTRACE=full

    // ------------------------------------------------------------
    // Neat Error Handling
    // ------------------------------------------------------------
    let file_path = "./Cargo.loo";
    let file = match File::open(&file_path) {
        Ok(file) => Some(file),
        Err(err) => {
            println!("Error: {:?}", err);
            None
        }
    };
    println!("{:?}", file);

    let file_path = "hello.txt";
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(mut fc) => {
                    fc.write_all(b"Hello world")
                        .expect("Can not Write into the file");
                    fc
                }
                Err(e) => panic!("Can not create a new file because: {:?}", e),
            },
            err_code => panic!("Problem Creating file: {:?}", err_code),
        },
    };

    file.rewind().expect("Can not wind"); // reading file agin after writng to it if we do not do
    // that rust can not read the new changes in the file
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Can not read form the file");
    println!("File content: {}", file_content);

    // ---------------------------------------
    // Skiping matchin with `unwrap`, `exept`:
    // Sometimes we need just write a simple statement with no regrious matching just panic on
    // errors
    // ---------------------------------------
    let file = File::open("hello.txt").unwrap();
    println!("{:?}", file);
    let file = File::open("hello.txt").expect("There is an error but I'm lazy to trace it");
    println!("{:?}", file);

    // ----------------------------------------
    // Error Propagation:
    // Propagating the error or just passing the error as it is by the other function
    // ----------------------------------------
    fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
        let mut file: File = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => return Err(e), // NOTE: we return here if and error happend
        };

        let mut username = String::new();
        match file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    read_username_from_file("hello.txt");

    // NOTE: Exact smae logic but with operator ?
    fn read_username_from_file_short(file_path: &str) -> Result<String, io::Error> {
        let mut file: File = File::open(&file_path)?; // NOTE: returnign error if error goes here
        let mut username = String::new();
        file.read_to_string(&mut username)?; // NOTE: retruning error if error goes here
        Ok(username)
    }
    read_username_from_file_short("hello.txt");
    // ----------------------------------------
    // Error Propagatin: ? operator
    // The operator `?` calls the form function from train Form which converty types from the
    // io::Error to our custom error type.
    // To support the ? operator over our custom error we have to implement the from function from
    // the `From` trait
    // ----------------------------------------
    fn read_username_from_file_very_very_short(filepath: &str) -> Result<String, io::Error> {
        let mut username = String::new();
        File::open(&filepath)?.read_to_string(&mut username)?;
        Ok(username)
    }
    read_username_from_file_very_very_short("hello.txt");
    // Doing the exact sameting as the above
    fn read_username_from_file_std(filepath: &str) -> Result<String, io::Error> {
        fs::read_to_string(&filepath)
    }
}
