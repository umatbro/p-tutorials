use itertools::Itertools;
use packet::Pair;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod packet;
mod parse;

fn main() {
    let is_test = false;
    let filename = if is_test { "input.test" } else { "input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut index = 1;

    for (left, right, _) in reader.lines().tuples() {
        // dbg!(&left, &right, index);
        let pair = Pair::from(&left.unwrap(), &right.unwrap());
        let in_order = pair.in_order();
        println!("{}", in_order);
        if in_order {
            result += index;
        }
        index += 1;
    }

    println!("Part 1 result is {result}.");
}
