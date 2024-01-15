mod task;
mod hailstone;
mod test_area;

use std::str::FromStr;

use hailstone::{Hailstone, Intersection};
use test_area::TestArea;


fn part1(file_content: &String, test_area: &TestArea) -> u64 {
    let hailstones = file_content
        .lines()
        .map(|line| Hailstone::from_str(line).unwrap())
        .collect::<Vec<Hailstone>>();
    // iterate over all possible pairs of hailstones
    // and check how many intersect
    let mut result = 0;
    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            let intersection = hailstones[i].find_interesection(&hailstones[j], &test_area);
            match intersection {
                Intersection::InArea(_) => result += 1,
                _ => (),
            }
        }
    }
    result
    
}

fn main() {
    let content = task::read_task("input");
    let test_area = TestArea::new(200000000000000, 400000000000000);
    let result = part1(&content, &test_area);
    println!("Result part1: {}", result);
}


#[cfg(test)]
mod tests {
    use crate::task;
    use super::part1;

    #[test]
    fn test_part1() {
        let content = task::read_task("input.test");
        let test_area = super::TestArea::new(7, 27);
        let result = part1(&content, &test_area);
        assert_eq!(result, 2);
    }
}
