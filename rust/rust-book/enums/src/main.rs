enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    NorthCarolina,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    let result = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    };
    result
}

fn main() {
    println!("Hello, world!");
    let var1 = Option::Some(5);
    let var2 = Option::<i32>::None;

    let result1 = multiply_optional(var1, 6);
    let result2 = multiply_optional(var2, 3);
    println!("Result 1: {:?}", result1);
    println!("Result 2: {:?}", result2);

    let coin = Coin::Dime;
    println!("Dime value in cents: {}", value_in_cents(coin));
    let max_result = Some(10);
    if let Some(max_found) = max_result {
        println!("Max found: {}", max_found);
    }
}

fn multiply_optional(num: Option::<i32>, multiply_by: i32) -> Option::<i32> {
    match num {
        Option::Some(i) => Some(i + multiply_by),
        Option::None => Option::None,
    }
}

// This is also in the standard library, so I'm commenting this out
// enum Option<T> {
//     None,
//     Some(T),
// }