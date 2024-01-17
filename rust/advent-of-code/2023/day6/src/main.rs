use std::str::FromStr;

mod race;
mod task;

fn part1(races: &race::Races) -> u64 {
    races
        .0
        .iter()
        .map(|record| record.ways_to_win())
        .reduce(|acc, x| acc * x)
        .unwrap()
}

fn part2(races: &race::Races) -> u64 {
    races
        .0
        .get(0)
        .expect("The indexs 0 should be present")
        .ways_to_win()
}

fn main() {
    let file_content = task::read_task("input");
    let races = race::Races::from_str(&file_content).unwrap();
    let result = part1(&races);
    println!("Result 1: {}", result);

    let part2_input = race::Races(vec![race::RaceRecord::new(46807866, 214117714021024)]);
    let result = part2(&part2_input);
    println!("Result 2: {}", result);
}

#[cfg(test)]
mod tests {
    use self::race::{RaceRecord, Races};

    use super::*;

    #[test]
    fn test_part1() {
        let file_content = task::read_task("input.test");
        let races = race::Races::from_str(&file_content).unwrap();
        assert_eq!(part1(&races), 288);
    }

    #[test]
    fn test_part2() {
        let races = Races(vec![RaceRecord::new(71530, 940200)]);
        let result = part2(&races);
        assert_eq!(result, 71503);
    }
}
