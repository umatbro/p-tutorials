enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Hello, world!");
    let var1 = Option::Some(5);
    let var2 = Option::<i32>::None;

    let result1 = multiply_optional(var1, 6);
    let result2 = multiply_optional(var2, 3);
    println!("Result 1: {:?}", result1);
    println!("Result 2: {:?}", result2);
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