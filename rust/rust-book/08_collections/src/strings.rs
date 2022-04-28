pub fn basic() {
    let data = "initial string content";

    let s = data.to_string();
    println!("{}", s);
}

pub fn concat() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    
    let s1 = String::from("Abra ");
    let s2 = String::from("kadabra");
    let s3 = s1 + &s2;
    println!("concat str {}", s3);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}!", tic, tac, toe);
    println!("{}", tic_tac_toe);
}

pub fn indexing() {
    let hello = String::from("Здравствуйте");
    println!("Russian length: {}", hello.len());
    let hindi = String::from("नमस्ते");

    // for char in hindi.chars() {
    //     println!("{}", char);
    // }
    println!("Chars: {:?}", hindi.chars());
}
