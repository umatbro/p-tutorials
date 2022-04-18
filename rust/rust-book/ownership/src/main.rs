fn main() {
    let mut s = String::from("Hello"); // s comes into scope
    s.push_str(", world");
    println!("{}", s);
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s); // <-- this is not possible
    let x = 5;
    makes_copy(x);
    println!("After copy {}", x);

    let s2 = gives_ownership();
    println!("S2 {}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("S3: {}", s3);

    println!(
        "Len s: {}, len abc: {}",
        calculate_len(&s3),
        calculate_len(&String::from("abc"))
    );

    let mut text_to_change = String::from("Heeh");
    println!("{}", change(&mut text_to_change));
    slice_type();
}

fn takes_ownership(s: String) -> String {
    // some_string comes into scope
    println!("String to print: {}", s);
    return s;
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(v: i32) {
    println!("this is a copy {}", v);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_len(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(text: &mut String) -> &mut String {
    text.push_str("I change you");
    text
}

fn slice_type() {
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