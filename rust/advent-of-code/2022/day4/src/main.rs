mod range;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use range::{parse_range, parse_row};

fn main() {
    let path = Path::new("input");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut result_part_2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (left, right) = parse_row(line.as_str());
        let left_set = parse_range(left);
        let right_set = parse_range(right);
        if left_set.is_subset(&right_set) || right_set.is_subset(&left_set) {
            result += 1;
        }

        if !left_set.is_disjoint(&right_set) {
            result_part_2 += 1;
        }
    }

    println!("Resut is {}", result);
    println!("Result for part 2 is {}", result_part_2);
}
