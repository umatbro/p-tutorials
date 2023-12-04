use std::{num::NonZeroI128, str::FromStr};

use nom::{IResult, combinator::{self, opt}, bytes::complete::tag, character::complete::{space1, digit1}, number::complete::be_i32, branch};
use nom::sequence::tuple;

pub struct Register {
    cycle: u32,
    value: i32,
    sprite: Sprite,
    pixels: [[bool; 40]; 6],

    signal_strengths: Vec<i32>,
}

impl Register {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            value: 1,
            signal_strengths: Vec::new(),
            sprite: Sprite::new(),
            pixels: [[false; 40]; 6],
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.incr_cycle(),
            Instruction::Addx(v) => {
                self.incr_cycle();
                self.incr_cycle();
                self.value += v;
                self.move_sprite(v);
            }
        }
    }

    fn incr_cycle(&mut self) {
        self.draw_pixel();
        self.cycle += 1;

        match self.cycle {
            num if num % 40 == 20 => self.signal_strengths.push(self.value * num as i32),
            _ => {},
        };
    }

    fn draw_pixel(&mut self) {
        let (pixel_row, pixel_pos) = (self.cycle / 40, self.cycle % 40);
        let sprite_val = self.sprite.slice[pixel_pos as usize];
        self.pixels[pixel_row as usize][pixel_pos as usize] = sprite_val;
    }

    fn move_sprite(&mut self, v: i32) {
        self.sprite.reposition(v as isize);
    }

    pub fn part_1_result(&self) -> i32 {
        self.signal_strengths.iter().sum()
    }
}

impl ToString for Register {
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(40 * 6);
        for row in self.pixels.iter() {
            for ch in row.iter() {
                let pix = if *ch {'#'} else {'.'};
                result.push(pix);
            }
            result.push('\n');
        }

        result
    }
}

struct Sprite {
    slice: [bool; 40],
    pos: isize,
}

impl Sprite {
    fn new() -> Self {
        let mut sprite = [false; 40];
        for i in 0..3 {
            sprite[i] = true;
        }
        Self {
            slice: sprite,
            pos: 1,
        }
    }

    fn reposition(&mut self, offset: isize) {
        use std::cmp::{max, min};
        let mut new_slice = [false; 40];
        
        self.pos += offset;
        let lower = min(39, max(0, self.pos - 1));
        let upper = min(39, max(0, self.pos + 1));;
        let rng = lower..=upper;
        for i in rng {
            new_slice[i as usize] = true
        }
        self.slice = new_slice;
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = branch::alt((parse_noop, parse_addx))(s);
        match res {
            Ok((s, instruction)) => Ok(instruction),
            Err(e) => Err(format!("There was an error: {}", e)),
        }
    }
}

fn parse_noop(input: &str) -> IResult<&str, Instruction>{
    combinator::map(tag("noop"), |_| Instruction::Noop)(input)
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, rhs) = tuple((tag("addx"), space1, opt(tag("-")), digit1))(input)?;
    let (_, _, sign, digits) = rhs;
    let number = digits.parse::<i32>().unwrap();
    let sign = match sign {
        Some("-") => -1,
        _ => 1,
    };
    Ok((input, Instruction::Addx(sign * number)))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    use rstest::*;
    use super::Register;
    use super::Instruction;
    use Instruction::*;

    #[test]
    fn test_perform_instruction() {
        let mut r = Register::new();
        r.execute(Noop);
        assert_eq!(r.value, 1);
        assert_eq!(r.cycle, 1);
        r.execute(Addx(-100));
        assert_eq!(r.cycle, 3);
        assert_eq!(r.value, -99);
    }

    #[rstest]
    #[case("noop", Ok(Noop))]
    #[case("addx 0", Ok(Addx(0)))]
    #[case("addx -10", Ok(Addx(-10)))]
    #[case("addx 69", Ok(Addx(69)))]
    fn test_parse_instruction(#[case] input: &str, #[case] expected_out: Result<Instruction, String>) {
        let result = input.parse::<Instruction>();

        assert_eq!(result, expected_out);
    }
    
    #[test]
    fn test_example_case() {
        let file = File::open("input.test").unwrap();
        let reader = BufReader::new(file);
        
        let mut register = Register::new();

        for l in reader.lines() {
            let line = l.unwrap();
            let instruction = line.parse::<Instruction>().unwrap();
            register.execute(instruction);
        }

        assert_eq!(register.signal_strengths, vec![420, 1140, 1800, 2940, 2880, 3960]);
        assert_eq!(register.part_1_result(), 13140);
        let to_str = register.to_string();
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(to_str, expected);
    }
}
