use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = Path::new(&config.filename);

    if !file.exists() {
        println!("File {:?} does not exist", file);
        process::exit(1);
    }

    let contents = fs::read_to_string(file)?;
    println!("With text:\n{}", contents);
    Ok(())
}
