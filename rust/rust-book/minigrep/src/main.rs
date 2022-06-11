use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args are {:?}", args);
    let substr: &String = args.get(1).expect("Substring should be provided");
    let file: &String = args.get(2).expect("Filename should be provided");
    if !Path::new(file).exists() {
        panic!("File {} does not exist", file);
    }
}
