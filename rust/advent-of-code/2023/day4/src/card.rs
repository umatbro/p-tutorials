use std::{collections::{HashSet, HashMap}, str::FromStr};
use itertools::Itertools;

type CardNumber = u32;
type NumberOfCards = u32;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub card_number: CardNumber,
    pub winning: HashSet<u32>,
    pub bets: HashSet<u32>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");
        let game_number = split.next().unwrap();
        let numbers = split.next().unwrap();

        let to_parse = game_number.replace("Card", "").replace(" ", "");
        let card_number = to_parse.parse::<u32>().unwrap();
        
        let mut split = numbers.split(" | ");
        // handle multiple spaces
        let winning = split.next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<u32>>();
        let bets = split.next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<u32>>();
        Ok(Card { card_number, winning, bets })      
    }
}

impl Card {
    pub fn get_number_of_matching_numbers(&self) -> NumberOfCards {
        self.winning.intersection(&self.bets).count() as u32
    }

    pub fn score(&self) -> u32 {
        match self.get_number_of_matching_numbers() {
            0 => 0,
            v => 2u32.pow(v - 1),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CardPile<'a> {
    pile_stats: HashMap<CardNumber, NumberOfCards>,
    cards: HashMap<CardNumber, &'a Card>,
}

impl <'a> CardPile <'a> {
    pub fn new(cards: &'a Vec<Card>) -> Self {
        let mut pile_stats = HashMap::with_capacity(cards.len());
        let mut cards_ = HashMap::with_capacity(cards.len());

        for card in cards {
            let card_number = card.card_number;
            let number_of_cards = pile_stats.get(&card_number).unwrap_or(&0) + 1;
            pile_stats.insert(card_number, number_of_cards);
            cards_.insert(card_number, card);
        }
        Self { pile_stats, cards: cards_ }
    }

    pub fn analyze(&mut self) -> Result<(), String> {

        for card_num in self.cards.keys().sorted() {
            let card = self.cards.get(card_num).unwrap();
            let matching = card.get_number_of_matching_numbers();
            let sets_to_add = self.pile_stats.get(card_num).unwrap().clone();
            let indices_to_increment = card_num+1..=(card_num + matching) as u32;

            for index in indices_to_increment {
                let number_of_cards = self.pile_stats.get(&index).unwrap();
                let number_of_cards = number_of_cards + sets_to_add;
                self.pile_stats.insert(index, number_of_cards);
            }
        }
        Ok(())
    }

    pub fn get_total_number_of_cards(&self) -> NumberOfCards {
        self.pile_stats.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::collections::HashSet;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        Card { card_number: 1, winning: HashSet::from([41, 48, 83, 86, 17]), bets: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]) }
    )]
    #[case(
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        Card { card_number: 2, winning: HashSet::from([13, 32, 20, 16, 61]), bets: HashSet::from([61, 30, 68, 82, 17, 32, 24, 19]) }
    )]
    #[case(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        Card { card_number: 3, winning: HashSet::from([1, 21, 53, 59, 44]), bets: HashSet::from([69, 82, 63, 72, 16, 21, 14, 1]) }
    )]
    #[case(
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        Card { card_number: 4, winning: HashSet::from([41, 92, 73, 84, 69]), bets: HashSet::from([59, 84, 76, 51, 58, 5, 54, 83]) }
    )]
    #[case(
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        Card { card_number: 5, winning: HashSet::from([87, 83, 26, 28, 32]), bets: HashSet::from([88, 30, 70, 12, 93, 22, 82, 36]) }
    )]
    #[case(
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        Card { card_number: 6, winning: HashSet::from([31, 18, 13, 56, 72]), bets: HashSet::from([74, 77, 10, 23, 35, 67, 36, 11]) }
    )]
    fn test_parse_card(#[case] input: &str, #[case] expected: Card) {
        let card = input.parse::<Card>().unwrap();
        assert_eq!(card, expected);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_score(#[case] input: &str, #[case] expected: u32) {
        let card = input.parse::<Card>().unwrap();
        assert_eq!(card.score(), expected);
    }

    #[test]
    fn test_analyze() {
        let cards = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse::<Card>().unwrap(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".parse::<Card>().unwrap(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".parse::<Card>().unwrap(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".parse::<Card>().unwrap(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".parse::<Card>().unwrap(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".parse::<Card>().unwrap(),
        ];
        let mut card_pile = CardPile::new(&cards);
        card_pile.analyze().unwrap();
        assert_eq!(card_pile.pile_stats, HashMap::from_iter(vec![
                (1, 1),
                (2, 2),
                (3, 4),
                (4, 8),
                (5, 14),
                (6, 1),
            ])
        );
        assert_eq!(card_pile.get_total_number_of_cards(), 30);
    }
}