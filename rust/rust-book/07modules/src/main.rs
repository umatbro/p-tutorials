mod front_of_house;


mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                _seasonal_fruit: String::from("peach"),
            }
        }
    }

    pub enum Apetizer {
        Soup,
        Salad,
    }
}

fn eat_at_resteurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    
    use crate::back_of_house::Breakfast;
    use self::back_of_house::Apetizer;
    // this can be also done this way:
    use crate::back_of_house::{Apetizer as Apetizer2, Breakfast as Breakfast2};
    
    let bf = Breakfast::summer("Dry bread");
    Breakfast2::summer("Suchar");
    println!("Im gonna eat {:?}", bf);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // bf.seasonal_fruit = String::from("blueberries");
    let _order1 = Apetizer::Soup;
    let _order2 = Apetizer2::Salad;
}

fn main() {
    eat_at_resteurant();
}
