mod register;
use std::fs::File;
use std::io::{BufReader, BufRead};
use register::Instruction;
use register::Register;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let mut register = Register::new();

    for l in reader.lines() {
        let line = l.unwrap();
        let instruction = line.parse::<Instruction>().unwrap();
        register.execute(instruction);
    }
    println!("Part 1 result is {}", register.part_1_result());
    println!("Part 2 result is:\n{}", register.to_string());
}
