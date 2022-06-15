use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args are {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let query: &String = args.get(1).expect("Query string should be provided");
    // let file: &String = args.get(2).expect("Filename should be provided");

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let file = Path::new(&config.filename);

    if !file.exists() {
        panic!("File {:?} does not exist", file);
    }

    let contents = fs::read_to_string(file).expect("Something went wrong when reading the file.");
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
