mod mapper;
mod task;

use std::str::FromStr;
use rayon::prelude::*;

use mapper::MapperChain;


fn part1(file_content: &str) -> u64 {
    let mapper_chain = MapperChain::from_str(file_content).expect("Invalid mapper chain");
    mapper_chain.list_of_seeds
        .iter()
        .map(|seed| mapper_chain.map_seed(*seed))
        .min().unwrap()
}

fn part2(file_content: &str) -> u64 {
    let mapper_chain = MapperChain::from_str(file_content).expect("Invalid mapper chain");
    let seeds_to_check = mapper_chain.list_of_seeds
        .chunks(2)
        .map(|chunk| {
            let (start_seed, length) = (chunk[0], chunk[1]);
            let range_to_check = start_seed..(start_seed + length);
            range_to_check
        })
        .flatten().collect::<Vec<u64>>();
    println!("Collected seeds to check: {:?}", seeds_to_check.len());
    seeds_to_check.par_iter()
        .map(|seed| mapper_chain.map_seed(*seed))
        .min()
        .unwrap()
}

fn main() {
    let file_content = task::read_task("input");
    let result = part1(&file_content);
    println!("Result part 1: {}", result);

    let result = part2(&file_content);
    println!("Result part 2: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file_content = task::read_task("input.test");
        let result = part1(&file_content);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let file_content = task::read_task("input.test");
        let result = part2(&file_content);
        assert_eq!(result, 46);
    }
}