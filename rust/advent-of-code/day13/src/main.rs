use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;
use packet::Pair;

mod parse;
mod packet;

fn main() {
    let is_test = true;
    let filename = if is_test {"input.test"} else {"input"};
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut result = 0;

    for (left, right, _) in reader.lines().tuples() {
        index += 1;
        let pair = Pair::from(&left.unwrap(), &right.unwrap());
        if pair.in_order() {
            result += 1;
        }
    }

    println!("Part 1 result is {result}.");
}
