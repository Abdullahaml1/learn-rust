use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    #[derive(Debug, Clone, Copy, EnumIter, Hash, PartialEq, Eq)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            // NOTE: closure
            // clousar function example
            user_preference.unwrap_or_else(|| self.most_stoked())
        }
        fn most_stoked(&self) -> ShirtColor {
            let mut counts: HashMap<ShirtColor, u64> = HashMap::new();
            for color in ShirtColor::iter() {
                counts.insert(color, 0);
            }
            for item_color in &self.shirts {
                *counts.get_mut(item_color).unwrap() += 1
            }
            let mut max_color_count = 0;
            let mut max_color = ShirtColor::Red;

            for (color, val) in &counts {
                if *val > max_color_count {
                    max_color_count = *val;
                    max_color = *color;
                }
            }
            max_color
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // --------------------------------------------------------
    // closur
    // --------------------------------------------------------
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;

    // --------------------------------------------------------
    // closurs can use external varibles like lambda functions in python
    // --------------------------------------------------------
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    // this will not take ownershipe over the varialbe v (just borrows it)
    let mut push_closur = || v.push(5);
    push_closur();
    println!("{:?}", v);

    // if you wish to force moving the variable in the enviroment into the clousr use the
    // `move` key work
    let mut push_closur_move = move || v.push(5);
    // println!("{:?}", v); // BUG: v has moved

    // --------------------------------------------------------
    // Types of closurs:
    //   | Trait Name | Descriptiion                                                        |
    //   |------------|---------------------------------------------------------------------|
    //   | FnOnce     | a function that is used once                                        |
    //   | FnMute     | can be used many times and can mutate enviroment varaibles          |
    //   | Fn         | can be used many times **without modifing the enviroment varialbles |
    // --------------------------------------------------------
    // impl<T> Option<T> {
    //     pub fn my_unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T,
    //     {
    //         match (self) {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    // -------------------------------------------------------------
    // Fnmute Example
    // -------------------------------------------------------------
    #[derive(Debug)]
    struct Rectangle {
        width: u64,
        height: u64,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 20,
        },
        Rectangle {
            width: 3,
            height: 4,
        },
        Rectangle {
            width: 7,
            height: 7,
        },
    ];

    let val = String::from("how are you");
    let mut counter = 0;

    list.sort_by_key(|r| {
        // drop(val); // BUG: we can not move val as this is a FnMute
        counter += 1;
        -((r.width * r.height) as i64)
    });
    println!("{:?}, operations: {}", list, counter);
    // we have 6 operations o sorting
}
