use std::{fs::File, io::{BufReader, BufRead}};

mod pos;
mod rope;
mod parse;

use rope::Rope;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut rope = Rope::new(2);
    let mut rope2 = Rope::new(10);

    for l in reader.lines() {
        let line = l.unwrap();
        let directions_list = parse::parse_line(&line).unwrap();
        let directions_for_part_2 = directions_list.clone();

        for direction in directions_list {
            rope.make_move(direction);
        }
        for direction in directions_for_part_2 {
            rope2.make_move(direction);
        }
    }

    println!("Result for part 1 is {}", rope.get_tail_positions().len());
    println!("Result for part 2 is {}", rope2.get_tail_positions().len());
}
