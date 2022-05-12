mod panic;
mod errors;
mod propagating_errors;

fn main() {
    // panic::panic();
    panic::vec_panic();

    // errors::err();
    // errors::handle_error();
    let username = propagating_errors::read_username_from_file().expect("Username could not be read");
    println!("Username is {}", username);
    let username = propagating_errors::read_username_q_operator().expect("Username could not be read");
    println!("Username is {}", username);
}
