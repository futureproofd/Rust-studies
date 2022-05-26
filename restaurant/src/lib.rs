// crate top level

// Declaring the front_of_house module whose body will be in src/front_of_house.rs
mod front_of_house;

// module
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // note that because back_of_house::Breakfast has a private field, the struct needs to provide
    // a public associated function that constructs an instance of Breakfast
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                // private
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // super, like using ../ to go to parent
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// bring path into scope
// we stop at the parent, hosting, rather than bringing the actual function
// name into scope, to make it clear that it's from somewhere else
pub use crate::front_of_house::hosting;
// we can also bring in two modules and alias one to differentiate
use std::fmt::Result;
use std::io::Result as IoResult;
// nested paths for items in the same scope
use std::{cmp::Ordering, io};
// glob operator:
use std::collections::*; // be careful as this makes it harder to tell what names are in scope.
                         // mostly used for testing

// public API of library crate
pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // can change pub
    meal.toast = String::from("wheat");
    println!("Id like {} toast", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
