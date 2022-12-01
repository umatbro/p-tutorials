use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};


#[derive(Debug)]
struct Elf {
    total_calories: u64,
}

fn main() {
    let path = Path::new("input");
    let file = File::open(path).unwrap();

    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut elves: Vec<Elf> = Vec::new();
    let mut calorie_counter = 0;

    while let Some(line) = lines.next() {
        let line_content = line.unwrap();
        let line_content = line_content.trim();
        if line_content.is_empty() {
            elves.push(Elf {total_calories: calorie_counter.clone()});
            calorie_counter = 0;
            continue;
        }
        let parse = line_content.parse::<u64>();
        match parse {
            Ok(val) => calorie_counter += val,
            Err(_) => eprintln!("Could not parse value {}", line_content),
        }
    }

    println!("Max calories are: {}", elves.iter().map(|el| el.total_calories).max().unwrap());

    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    let result: u64 = elves.iter().take(3).map(|elf| elf.total_calories).sum();
    println!("Top 3 calories are: {:?}", result);
}