fn pass_order_to_kitchen() {
    println!("Passing order!")
}

pub mod hosting {
    pub fn add_to_waitlist() {
        super::pass_order_to_kitchen();
    }
}