use itermore::IterArrayChunks;
use monke::MonkeBuilder;
use monke::{Monke, MonkeInspection};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

mod monke;
mod parse;
use parse::{
    if_false_target_parser, if_true_target_parser, parse_divisible_by, parse_starting_items,
};

fn main() {
    let file = File::open("input").unwrap();
    // let file = File::open("input.test").unwrap();
    let reader = BufReader::new(file);

    let mut monkeys: Vec<Monke> = vec![];

    for cfg in reader.lines().map(|l| l.unwrap()).array_chunks::<7>() {
        let monkey_items = parse_starting_items(&cfg[1]).unwrap().1;

        let m_builder = MonkeBuilder::new()
            .with_items(monkey_items.into_iter())
            .set_inspection_rules(cfg[2].parse::<MonkeInspection<u32>>().unwrap())
            .set_divisible_by(parse_divisible_by(&cfg[3]).unwrap().1);

        let (_, if_true) = if_true_target_parser(&cfg[4]).unwrap();
        let (_, if_false) = if_false_target_parser(&cfg[5]).unwrap();
        let m_builder = m_builder.set_throw_to_monke(if_true, if_false);
        monkeys.push(m_builder.build().unwrap());
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut current_monke_items: VecDeque<u32> = VecDeque::new();
            let monke = monkeys.get_mut(i).unwrap();
            for i in monke.items.iter() {
                current_monke_items.push_back(i.clone());
            }
            monke.items = VecDeque::new();
            drop(monke);
            while let Some(current_item) = current_monke_items.pop_front() {
                let monke = monkeys.get_mut(i).unwrap();
                let (throw_to, value_of_item) = monke.inspect(&current_item);
                drop(monke);
                // println!("I'm monkey {} and I want to throw {} to monkey {}", i, value_of_item, throw_to);
                let target = monkeys.get_mut(throw_to).unwrap();
                target.items.push_back(value_of_item);
            }
        }
        // print_what_monkeys_hold(&monkeys, round);
    }

    let mut numbers_of_inspections: Vec<u32> = monkeys
        .iter()
        .map(|m| m.get_number_of_inspections())
        .collect();
    println!("Number of inspections {:?}", numbers_of_inspections);
    numbers_of_inspections.sort_by(|a, b| b.cmp(a));
    println!(
        "Result for part 1 is {:?}",
        numbers_of_inspections
            .iter()
            .take(2)
            .fold(1, |acc, &x| acc * x)
    )
}

fn print_what_monkeys_hold(mnks: &Vec<Monke>, round_number: usize) {
    println!("Round {}", round_number);
    for (i, mnk) in mnks.iter().enumerate() {
        println!("Monkey {} holds: {:?}", i, mnk.items);
    }
}
