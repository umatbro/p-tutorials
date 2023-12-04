use std::{path::Path, fs::File, io::{BufReader, BufRead}, ops::{Add, AddAssign}, collections::binary_heap::Drain};

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
struct Points(u32);
impl Add for Points {
    type Output = Points;

    fn add(self, rhs: Self) -> Self::Output {
        Points(self.0 + rhs.0)
    }
}
impl AddAssign for Points {
    fn add_assign(&mut self, rhs: Self) {
        let result = self.0 + rhs.0;
        self.0 = result;
    }
}

trait ToPoints {
    fn to_points(&self) -> Points;
}

impl ToPoints for Choice {
    fn to_points(&self) -> Points {
        use Choice::*;

        match self {
            Rock => Points(1),
            Paper => Points(2),
            Scissors => Points(3),
        }
    }
}

impl Choice {
    fn respond_with(&self, response: &Choice) -> GameResult {
        use Choice::*;
        use GameResult::*;

        match (self, response) {
            (Rock, Paper) => Win,
            (Paper, Rock) => Loss,
            (Rock, Scissors) => Loss,
            (Scissors, Rock) => Win,
            (Paper, Scissors) => Win,
            (Scissors, Paper) => Loss,
            _ => Draw,
        }
    }

    fn parse<T: ToString>(val: T) -> Result<Choice, ()> {
        use Choice::*;

        match val.to_string().as_str() {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }

    fn my_choice_needed_to_game_result(&self, game_result: &GameResult) -> Choice {
        use Choice::*;
        use GameResult::*;

        match (self, game_result) {
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Loss) => Rock,
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Loss) => Scissors,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Loss) => Paper,
        }
    }

    fn parse_game_result<T: ToString>(val: T) -> Result<GameResult, ()> {
        use GameResult::*;

        match val.to_string().as_str() {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

impl ToPoints for GameResult {
    fn to_points(&self) -> Points {
        use GameResult::*;

        match self {
            Win => Points(6),
            Draw => Points(3),
            Loss => Points(0),
        }
    }
}

struct Game {
    opponent: Choice,
    my_choice: Choice,
}

impl Game {
    fn parse_line(line: &str) -> Self {
        let vec = line.split_whitespace().take(2).collect::<Vec<&str>>();

        let opponent = vec[0];
        let me = vec[1];
        Game {
            opponent: Choice::parse(opponent).unwrap(),
            my_choice: Choice::parse(me).unwrap(),
        }
    }

    fn play(&self) -> Points {
        let mut result = Points(0);

        result += self.my_choice.to_points();
        result += self.opponent.respond_with(&self.my_choice).to_points();
        result
    }
}

struct GameMethod2 {
    opponent: Choice,
    expected_outcome: GameResult,
}

impl GameMethod2 {
    fn parse_line(line: &str) -> Self {
        let vec = line.split_whitespace().take(2).collect::<Vec<&str>>();

        let opponent = vec[0];
        let expected_outcome = vec[1];

        GameMethod2 { opponent: Choice::parse(opponent).unwrap(), expected_outcome: Choice::parse_game_result(expected_outcome).unwrap() }
    }

    fn play(&self) -> Points {
        let mut result = Points(0);

        result += self.expected_outcome.to_points();
        result += self.opponent.my_choice_needed_to_game_result(&self.expected_outcome).to_points();
        result
    }
}

fn main() {
    let path = Path::new("input");
    let file = File::open(path).unwrap();

    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut total_score = Points(0);
    let mut total_score_method_2 = Points(0);

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let game = Game::parse_line(&line);
        let game_method_2 = GameMethod2::parse_line(&line);
        total_score += game.play();
        total_score_method_2 += game_method_2.play();
    }

    println!("Total score is: {}", total_score.0);
    println!("Total score for method 2 is: {}", total_score_method_2.0);
}
