use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args are {:?}", args);
    let query: &String = args.get(1).expect("Query string should be provided");
    let file: &String = args.get(2).expect("Filename should be provided");

    println!("Searching for {}", query);
    println!("In file {}", file);

    let file = Path::new(file);

    if !file.exists() {
        panic!("File {:?} does not exist", file);
    }

    let contents = fs::read_to_string(file).expect("Something went wrong when reading the file.");
    println!("With text:\n{}", contents);
}
