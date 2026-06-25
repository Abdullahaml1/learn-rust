// **Unit Internal tests**
//
//
// ----------------------------------------------
// NOTE: Tsts run in parallel by default to run them sequential run:
// $ cargo test -- --test-threads=1
// ----------------------------------------------
//
// ----------------------------------------------
//  cargo  tests showts the function ourput (println!) if the function failed.
//  To show all the output of all functions run:
//  $ cargo test -- --show-output
// ----------------------------------------------

// ----------------------------------------------
//  to run group of tests that begin with prefix for example `it_works`
//  $ cargo test it_works
// ----------------------------------------------

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    // format!("Hello how are you {}!", name)
    format!("Hello how are you !")
}

pub fn just_panic() {
    panic!("Do not get here");
}
// ----------------------------------------------------------------
// custom structs and enums have to implement `#[derive(PartialEq, Debug)]` in order to be printed
// with assert
// ----------------------------------------------------------------

// `cfg` stands for configuration
#[cfg(test)]
mod tests {
    use std::{error::Error, result};

    use super::*; // using all uppoer modules (ansector modules)

    #[test]
    fn it_works() {
        let result = add(2, 2);
        // We have
        // assert!
        // assert_eq!
        // assert_ne!
        assert_eq!(result, 4);
    }
    #[test]
    fn this_is_an_error() {
        panic!("Just panic")
    }

    #[test]
    fn greetings_contain_name() {
        let result = greeting("Hamo");
        assert!(
            result.contains("Hamo"),
            "There is and error executing the test the input is: {} expected: {}",
            "Hamo",
            "Hellow how are you Hamo!"
        );
    }

    //-------------------------------------------------------
    // `should_panic` similar to pythos (should raise)
    //-------------------------------------------------------
    #[test]
    #[should_panic]
    fn this_should_panic() {
        just_panic();
    }
    //-------------------------------------------------------
    // `should_panic` with specific message
    //-------------------------------------------------------
    #[test]
    #[should_panic(expected = "Do not get here")]
    fn this_should_panic_with_msg() {
        just_panic();
    }

    //-------------------------------------------------------
    // Instead of panic we can return result with error and Ok
    //-------------------------------------------------------
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        let result = add(2, 2);
        match result {
            4 => Ok(()),
            _ => Err("This have to be four".to_string()),
        }
    }

    //-------------------------------------------------------
    // Ignoring tests unless requested
    //-------------------------------------------------------
    #[test]
    #[ignore]
    fn ignore_this() {
        println!("works");
    }
}
