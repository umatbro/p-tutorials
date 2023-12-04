use std::{fs::File, io::{BufReader, BufRead}};

mod parser;
mod forest;
use forest::Forest;

fn main() {
    let file = File::open("input").unwrap();
    // let file = File::open("input.test").unwrap();
    let reader = BufReader::new(file);

    let mut forest = Forest::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let mut row: Vec<u32> = Vec::with_capacity(l.len());
        for ch in l.chars() {
            let parsed: u32 = ch.to_string().parse().unwrap();
            row.push(parsed);
        }
        forest.add_row(row);
    }

    println!("Part 1 result is {}", forest.count_visible());
    println!("Part 2 result is {}", forest.find_max_viewscore());
}
