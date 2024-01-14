mod task;
mod card;

fn part1(file_content: &String) -> u32 {
    let cards = file_content.lines().map(|line| line.parse::<card::Card>().unwrap()).collect::<Vec<card::Card>>();
    cards.iter().map(|card| card.score()).sum()
}

fn part2(file_content: &String) -> u32 {
    let cards = file_content.lines().map(|line| line.parse::<card::Card>().unwrap()).collect::<Vec<card::Card>>();
    let mut pile = card::CardPile::new(&cards);
    pile.analyze().unwrap();
    pile.get_total_number_of_cards()
}

fn main() {
    let file_content = task::read_task("input");
    let result = part1(&file_content);
    println!("Result 1: {}", result);

    let result = part2(&file_content);
    println!("Result 2: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_task() {
        let file_content = task::read_task("input.test");
        let result = part1(&file_content);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let file_content = task::read_task("input.test");
        let result = part2(&file_content);
        assert_eq!(result, 30);
    }
}
