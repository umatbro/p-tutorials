use std::{fs::File, io::{BufReader, BufRead}};
use std::io;
use board::{Board, Direction};

mod coord;
mod shape;
mod board;

fn main() {
    let is_test = true;
    let filename = if is_test {"input.test"} else{ "input"};
    let input = File::open(filename).unwrap();
    let reader = BufReader::new(input);

    let mut board = Board::new();

    let line = reader.lines().next().unwrap().unwrap();
    for c in line.chars().cycle() {
        // println!("{}\n-----", board);
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).expect("Failed to read line");
        if board.rocks_stopped % 100000 == 0 {
            println!("Rocks stopped: {}", board.rocks_stopped);
        }
        if board.rocks_stopped == 1000000000000 {
            break;
        }
        match c {
            '<' => board.play_round(Direction::Left),
            '>' => board.play_round(Direction::Right),
            v => panic!("Unexpected value {}", v),
        }
    }

    println!("Part 1 result is {}", board.get_max_height());
}
