mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// This line does not make the module accesible in the `customer` module we have to add this line
// in the customer module to be valid
use crate::front_of_house::hosting;
mod customer {

    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// -----------------------------------------------------
// use as keywork
// -----------------------------------------------------
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// -----------------------------------------------------
// The use stayes private in the current module unless yo make it public
// -----------------------------------------------------

mod front_of_house_1 {
    pub mod hosting_1 {
        pub fn add_to_waitlist() {}
    }
}

// Making the hostig_1 module public to our users
pub use crate::front_of_house_1::hosting_1;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// to import muliple libraries at the same time
use std::{cmp::Ordering, io};

// Brinign all items of specific modules
use std::collections::*;

