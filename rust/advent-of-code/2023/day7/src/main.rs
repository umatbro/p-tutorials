use std::fs;
mod camel_game;
use camel_game::Hand;

fn part1(file_contents: &str) -> u64 {
    let mut hands: Vec<Hand> = file_contents
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid as u64 * (i as u64 + 1))
}

fn main() {
    let file_contents = fs::read_to_string("input").expect("Error reading the file");
    println!("Part 1: {}", part1(&file_contents));
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_part1() {
        let file_content = fs::read_to_string("input.test").expect("Error reading the file");
        assert_eq!(super::part1(&file_content), 6440);
    }
}
