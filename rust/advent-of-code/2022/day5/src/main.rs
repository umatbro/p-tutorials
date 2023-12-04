use std::{str::FromStr, path::Path, fs::File, io::{BufReader, BufRead}};
use nom::{self, IResult};

//             [J] [Z] [G]            
//             [Z] [T] [S] [P] [R]    
// [R]         [Q] [V] [B] [G] [J]    
// [W] [W]     [N] [L] [V] [W] [C]    
// [F] [Q]     [T] [G] [C] [T] [T] [W]
// [H] [D] [W] [W] [H] [T] [R] [M] [B]
// [T] [G] [T] [R] [B] [P] [B] [G] [G]
// [S] [S] [B] [D] [F] [L] [Z] [N] [L]
//  1   2   3   4   5   6   7   8   9 
#[allow(dead_code)]
fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate().skip(1) {
        print!("{}: {:?}", i, stack);
        println!("");
    }
}

fn main() {
    let mut stacks = vec![
        vec![],
        vec!['S', 'T', 'H', 'F', 'W', 'R'],
        vec!['S', 'G', 'D', 'Q', 'W'],
        vec!['B', 'T', 'W'],
        vec!['D', 'R', 'W', 'T', 'N', 'Q', 'Z', 'J'],
        vec!['F', 'B', 'H', 'G', 'L', 'V', 'T', 'Z'],
        vec!['L', 'P', 'T', 'C', 'V', 'B', 'S', 'G'],
        vec!['Z', 'B', 'R', 'T', 'W', 'G', 'P'],
        vec!['N', 'G', 'M', 'T', 'C', 'J', 'R'],
        vec!['L', 'G', 'B', 'W'],
    ];
    let mut stacks_part_2 = stacks.clone();

    let path = Path::new("input");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        let s = l.parse::<StackTransfer>().unwrap();
        
        for _ in 0..s.amount {
            let source_stack = &mut stacks[s.from as usize];
            let el = source_stack.pop();
            if let Some(v) = el {
                let target_stack = &mut stacks[s.to as usize];
                target_stack.push(v);
            } else {
                panic!("The stack should not be empty!")
            }
        }

        // PART 2
        let source_stack = &mut stacks_part_2[s.from as usize];
        let amount: usize = s.amount as usize;
        let elements_to_move = source_stack[source_stack.len()-amount..].to_vec();
        *source_stack = source_stack[..source_stack.len()-amount].to_vec();
        let target_stack = &mut stacks_part_2[s.to as usize];
        target_stack.extend(elements_to_move);
        
    }
    let mut result1 = String::new();
    for stack in stacks.iter().skip(1) {
        let c = stack.last().unwrap().clone();
        result1.push(c);
    }
    println!("Result 1: {result1}");

    let mut result2 = String::new();
    for stack in stacks_part_2.iter().skip(1) {
        let c = stack.last().unwrap().clone();
        result2.push(c);
    }
    println!("Result 2: {result2}");
}

#[derive(Debug)]
struct StackTransfer {
    amount: u32,
    from: u32,
    to: u32,
}
fn parse_input(i: &str) -> IResult<&str, StackTransfer> {
    let (i, _) = nom::bytes::complete::tag("move")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, amount) = nom::combinator::map_res(nom::character::complete::digit1, u32::from_str)(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = nom::bytes::complete::tag("from")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, from) = nom::combinator::map_res(nom::character::complete::digit1, u32::from_str)(i)?;
    let (i, _) = nom::bytes::complete::tag(" to ")(i)?;
    let (i, to) = nom::combinator::map_res(nom::character::complete::digit1, u32::from_str)(i)?;
    Ok((i, StackTransfer {
        from,
        to,
        amount,
    }))
}

impl FromStr for StackTransfer {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_input(s).unwrap().1)
    }
}