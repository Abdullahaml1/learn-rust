mod front_of_house_1 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant_1() {
    // Absolute path
    crate::front_of_house_1::hosting::add_to_waitlist();

    // Relative path
    front_of_house_1::hosting::add_to_waitlist();
}

// ------------------------------------------------------------
// Chosign some fields of the struct to public / private
// ------------------------------------------------------------

mod back_of_house_2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house_2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}

// ------------------------------------------------------------
// IF you make Enum pulic all of its fields will be pbulic
// ------------------------------------------------------------
mod back_of_house_3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_3() {
    let order1 = back_of_house_3::Appetizer::Soup;
    let order2 = back_of_house_3::Appetizer::Salad;
}
