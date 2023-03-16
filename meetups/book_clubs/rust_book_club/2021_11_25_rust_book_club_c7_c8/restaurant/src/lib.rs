pub mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub use crate::front_of_house::serving;
    
    #[derive(PartialEq, Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn order(toast: &str, seasonal_fruit: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from(seasonal_fruit),
            }
        }
    }
}

pub fn eat_at_restaurant() -> back_of_house::Breakfast {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::order("Rye", "Peaches");

    meal.toast = String::from("Wheat"); // Fine
    // meal.seasonal_fruit = String::from("blueberries"); // ERROR
    meal
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn breakfast() {
        assert_eq!(super::eat_at_restaurant(), super::back_of_house::Breakfast::order("Wheat", "Peaches"));

    }
}
