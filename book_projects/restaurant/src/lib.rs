mod front_of_house;

mod back_of_house {
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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// using pub for outside code to use hosting directing if all parents of hosting are private
pub use crate::front_of_house::hosting;

mod customer {
    // can use hosting directly because above I am doing pub use crate::front_of_house::hosting;
    // which exports path below to all other files and module, each module have to use path
    // declaration to use another module
    use crate::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
    //

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//pub fn eat_at_restaurant() {
// // Absolute path
//crate::front_of_house::hosting::add_to_waitlist();

// //Relative path
//front_of_house::hosting::add_to_waitlist();
//}
