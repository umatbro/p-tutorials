mod engine;
mod task;

fn part1(task_content: &String) -> u32 {
    let engine = task_content.parse::<engine::Engine>().unwrap();
    engine
        .parts
        .iter()
        .filter(|part| part.has_symbol_in_adjacency(&engine.symbols))
        .map(|part| part.number)
        .sum()
}

fn part2(task_content: &String) -> u32 {
    let engine = task_content.parse::<engine::Engine>().unwrap();
    engine
        .symbols
        .iter()
        .map(|symbol| symbol.find_adjacent_parts(&engine.parts))
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[0].number * parts[1].number)
        .sum::<u32>()
}

fn main() {
    let task = task::read_task("input");
    println!("Part 1: {}", part1(&task));

    println!("Part 2: {}", part2(&task));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = task::read_task("input.test");
        assert_eq!(part1(&content), 4361);
    }

    #[test]
    fn test_part2() {
        let content = task::read_task("input.test");
        assert_eq!(part2(&content), 467835);
    }
}
