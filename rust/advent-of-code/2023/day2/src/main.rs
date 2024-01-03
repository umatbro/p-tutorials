use std::str::FromStr;

mod task;

#[derive(Debug, PartialEq, PartialOrd)]
struct Green(u16);
#[derive(Debug, PartialEq, PartialOrd)]
struct Red(u16);
#[derive(Debug, PartialEq, PartialOrd)]
struct Blue(u16);

const MAX_RED: Red = Red(12);
const MAX_GREEN: Green = Green(13);
const MAX_BLUE: Blue = Blue(14);


#[derive(Debug, PartialEq)]
struct Subset {
    green: Green,
    red: Red,
    blue: Blue,
}

impl FromStr for Subset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut green = Green(0);
        let mut red = Red(0);
        let mut blue = Blue(0);
        for part in s.split(", ") {
            let mut iter = part.split(" ");
            let number = iter.next().unwrap().parse::<u16>().unwrap();
            let color = iter.next().unwrap();
            match color {
                "green" => green = Green(number),
                "red" => red = Red(number),
                "blue" => blue = Blue(number),
                _ => return Err(format!("Could not parse subset: {}", s)),
            }
        }
        Ok(Subset { green, red, blue })
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    game_number: u16,
    subsets: Vec<Subset>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");
        let game_number = iter.next().unwrap().split(" ").nth(1).unwrap().parse::<u16>().unwrap();
        let subsets = iter.next().unwrap().split("; ").map(|s| Subset::from_str(s).unwrap()).collect();
        Ok(Game { game_number, subsets })
    }
}

impl Game {
    // game if possible if any of the subset does not contain the number bigger than the max
    fn is_possible(&self) -> bool {
        for subset in self.subsets.iter() {
            if subset.green > MAX_GREEN || subset.red > MAX_RED || subset.blue > MAX_BLUE {
                return false;
            }
        }
        true
    }

    fn get_power(&self) -> u16 {
        let mut max_green = None;
        let mut max_red = None;
        let mut max_blue = None;
        for subset in self.subsets.iter() {
            if max_green.is_none() || subset.green.0 > max_green.unwrap() {
                max_green = Some(subset.green.0);
            }
            if max_red.is_none() || subset.red.0 > max_red.unwrap() {
                max_red = Some(subset.red.0);
            }
            if max_blue.is_none() || subset.blue.0 > max_blue.unwrap() {
                max_blue = Some(subset.blue.0);
            }
        }

        max_green.unwrap() * max_red.unwrap() * max_blue.unwrap()
    }
}

fn part1(task_input: &String) -> u16 {
    let games = task_input.split("\n").map(|s| Game::from_str(s).unwrap()).collect::<Vec<Game>>();
    games
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.game_number)
        .sum::<u16>()
}

fn part2(task_input: &String) -> u32 {
    let games = task_input.split("\n").map(|s| Game::from_str(s).unwrap()).collect::<Vec<Game>>();
    games
        .iter()
        .map(|g| g.get_power() as u32)
        .sum::<u32>()
}


fn main() {
    let file_content = task::read_task("input");
    let p1_result = part1(&file_content);
    println!("Part 1 result: {}", p1_result);

    let p2_result = part2(&file_content);
    println!("Part 2 result: {}", p2_result);
}

#[cfg(test)]
mod tests {
    use crate::task;
    use crate::Game;
    use crate::Subset;
    use crate::{Blue, Green, Red};
    use rstest::*;
    use std::str::FromStr;

    #[test]
    fn test_part1() {
        let content = task::read_task("input-test.txt");
        let result = super::part1(&content);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2() {
        let content = task::read_task("input-test.txt");
        let result = super::part2(&content);

        assert_eq!(result, 2286);
    }

    #[rstest]
    #[case("3 blue, 4 red", Subset { green: Green(0), red: Red(4), blue: Blue(3) })]
    #[case("1 red, 2 green", Subset { green: Green(2), red: Red(1), blue: Blue(0) })]
    #[case("3 green, 15 blue, 14 red", Subset { green: Green(3), red: Red(14), blue: Blue(15) })]
    fn test_parse_subset(#[case] input: &str, #[case] expected: Subset) {
        let subset = Subset::from_str(input).unwrap();
        assert_eq!(subset, expected);
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 
        Game {
            game_number: 1,
            subsets: vec![
                Subset { green: Green(0), red: Red(4), blue: Blue(3) },
                Subset { green: Green(2), red: Red(1), blue: Blue(6) },
                Subset { green: Green(2), red: Red(0), blue: Blue(0) },
            ],
        }
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        Game {
            game_number: 4,
            subsets: vec![
                Subset { green: Green(1), red: Red(3), blue: Blue(6) },
                Subset { green: Green(3), red: Red(6), blue: Blue(0) },
                Subset { green: Green(3), red: Red(14), blue: Blue(15) },
            ],
        }
    )]
    fn test_parse_game(#[case] input: &str, #[case] expected: Game) {
        let game = Game::from_str(input).unwrap();
        assert_eq!(game, expected);
    }

    #[rstest]
    #[case(
        Game {
            game_number: 1,
            subsets: vec![
                Subset { green: Green(0), red: Red(4), blue: Blue(3) },
                Subset { green: Green(2), red: Red(1), blue: Blue(6) },
                Subset { green: Green(2), red: Red(0), blue: Blue(0) },
            ],
        },
        48,
    )]
    fn test_get_power(#[case] game: Game, #[case] expected_power: u16) {
        let power = game.get_power();
        assert_eq!(power, expected_power);
    }
}
