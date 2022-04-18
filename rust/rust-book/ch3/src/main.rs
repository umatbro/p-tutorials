// fn main() {
//     println!("Hello, world!");
//     let x = 5;
//     println!("X is {}", x);
// }
fn main() {
    let str = String::from("Hello world");
    let hello = &str[0..5];
    let world = &str[6..];
    println!("{}, {}", hello, world);
    println!("First word: {}", first_word(&str));
}

fn first_word(text: &String) -> &str {
    let bytes = text.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &text[..i];
        }
    }
    return &text[..];
}