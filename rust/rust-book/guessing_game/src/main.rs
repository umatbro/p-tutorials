use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn incr(num: u32) -> u32 {
    return num+1;
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);
    let mut counter = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed: {}", guess);
        counter = incr(counter);

        match guess.cmp(&secret_number) {
            Ordering::Less => {println!("Too small")},
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win after {} tries!", counter);
                break;
            }
        }
    }
}
