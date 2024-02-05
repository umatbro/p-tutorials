use std::{cmp::Ordering, collections::HashMap, fmt::Debug, iter::zip, str::FromStr};

#[derive(PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    pub bid: u32,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [
            Card::Number(0),
            Card::Number(0),
            Card::Number(0),
            Card::Number(0),
            Card::Number(0),
        ];
        let mut card_index = 0;
        let cards_part = s.split(" ").nth(0).ok_or("Invalid hand")?;
        let bid_part = s.split(" ").nth(1).ok_or("Invalid hand")?;
        for c in cards_part.chars() {
            cards[card_index] = c.to_string().parse::<Card>()?;
            card_index += 1;
        }
        let bid = bid_part
            .parse::<u32>()
            .map_err(|e| format!("Invalid bid: {}", e))?;
        Ok(Hand { cards, bid })
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<_> = self.cards.iter().map(|c| format!("{:?}", c)).collect();
        write!(f, "{} {}", cards.join(""), self.bid)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_hand_type = HandType::from(self);
        let other_hand_type = HandType::from(other);

        match self_hand_type.partial_cmp(&other_hand_type) {
            Some(Ordering::Equal) => {
                for (card_l, card_r) in zip(&self.cards, &other.cards) {
                    if card_l.partial_cmp(&card_r) != Some(Ordering::Equal) {
                        return card_l.partial_cmp(&card_r);
                    }
                }
                unreachable!("Cannot compare: {:?} and {:?}", self, other);
            }
            other => other,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    /// For part2, we need to map all Jacks to Jokers in a hand
    pub fn map_jacks_to_jokers(self) -> Self {
        let new_cards = self.cards.map(|c| match c {
            Card::Jack => Card::Joker,
            _ => c,
        });
        Self {
            cards: new_cards,
            bid: self.bid,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let scoring = |hand_type: &HandType| match hand_type {
            HandType::FiveOfAKind => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::ThreeOfAKind => 5,
            HandType::TwoPairs => 4,
            HandType::OnePair => 3,
            HandType::HighCard => 2,
        };
        let self_score = scoring(self);
        let other_score = scoring(other);
        self_score.partial_cmp(&other_score)
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&Hand> for HandType {
    fn from(hand: &Hand) -> Self {
        let mut number_of_cards: HashMap<&Card, u32> = HashMap::new();
        for card in hand.cards.iter() {
            number_of_cards
                .entry(card)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let number_of_jokers = number_of_cards.get(&Card::Joker).unwrap_or(&0);
        if number_of_jokers == &5 {
            return HandType::FiveOfAKind;
        }
        let max_number_of_cards: HashMap<_, _> = number_of_cards
            .iter()
            .filter(|(k, _)| **k != &Card::Joker)
            .map(|(k, v)| (*k, *v))
            .collect();
        
        // handle case when jokers are in the hand
        if number_of_jokers > &0 {
            let mut top2_repeated: Vec<_> = max_number_of_cards.values().collect();
            top2_repeated.sort();
            top2_repeated.reverse();
            let number_of_cards = match top2_repeated.as_slice() {
                [2, 2] => return HandType::FullHouse,
                v => v.iter().max().unwrap_or(&&0),
            };
            return match *number_of_cards + number_of_jokers {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => HandType::ThreeOfAKind,
                2 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }

        // handle case when there are no jokers in the hand
        let mut top2_repeated: Vec<_> = number_of_cards.values().collect();
        top2_repeated.sort();
        top2_repeated.reverse();

        let top2: Vec<_> = top2_repeated.iter().take(2).collect();
        match top2.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, _] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, _] => HandType::ThreeOfAKind,
            [2, 2] => HandType::TwoPairs,
            [2, _] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(PartialEq, Hash, Eq)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
    Joker,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Number(10)),
            _ => match s.parse::<u8>() {
                Ok(v) if v >= 2 && v <= 9 => Ok(Card::Number(v)),
                _ => Err(format!("Invalid card: {}", s)),
            },
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card: String = match self {
            Card::Ace => "A".into(),
            Card::King => "K".into(),
            Card::Queen => "Q".into(),
            Card::Jack => "J".into(),
            Card::Joker => "J".into(),
            Card::Number(10) => "T".into(),
            Card::Number(v) => format!("{}", v),
        };
        write!(f, "{}", card)
    }
}

impl From<&Card> for u32 {
    fn from(card: &Card) -> u32 {
        match card {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(v) => *v as u32,
            Card::Joker => 0,
        }
    }
}

// impl comparison of cards (for sorting)
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        let self_score: u32 = self.into();
        let other_score = other.into();
        self_score.partial_cmp(&other_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    enum SideIsGreater {
        Left,
        Right,
        Equal,
    }

    use Card::*;
    use SideIsGreater::*;

    #[rstest]
    #[case(Number(2), Number(3), Right)]
    #[case(Number(3), Number(4), Right)]
    #[case(Number(4), Number(5), Right)]
    #[case(Number(5), Number(6), Right)]
    #[case(Number(6), Number(7), Right)]
    #[case(Number(7), Number(8), Right)]
    #[case(Number(8), Number(9), Right)]
    #[case(Number(9), Number(10), Right)]
    #[case(Number(10), Jack, Right)]
    #[case(Jack, Queen, Right)]
    #[case(Queen, King, Right)]
    #[case(King, Ace, Right)]
    #[case(Number(3), Number(2), Left)]
    #[case(Ace, Number(3), Left)]
    #[case(King, King, Equal)]
    #[case(Number(5), Number(5), Equal)]
    fn test_card_ordering(#[case] left: Card, #[case] right: Card, #[case] side: SideIsGreater) {
        match side {
            Left => assert!(left > right),
            Right => assert!(right > left),
            Equal => assert_eq!(left, right),
        }
    }

    #[rstest]
    #[case("32T3K 765", Hand {
        cards: [Number(3), Number(2), Number(10), Number(3), King],
        bid: 765,
    })]
    #[case("T55J5 684", Hand {
        cards: [Number(10), Number(5), Number(5), Jack, Number(5)],
        bid: 684,
    })]
    #[case("KK677 28", Hand {
        cards: [King, King, Number(6), Number(7), Number(7)],
        bid: 28,
    })]
    fn test_parse_hand(#[case] hand: &str, #[case] expected: Hand) {
        let hand = hand.parse::<Hand>().unwrap();
        assert_eq!(hand, expected);
    }

    #[rstest]
    #[case(Hand::from_str("32T3K 765").unwrap(), HandType::OnePair)]
    #[case(Hand::from_str("T55J5 684").unwrap(), HandType::ThreeOfAKind)]
    #[case(Hand::from_str("KK677 28").unwrap(), HandType::TwoPairs)]
    #[case(Hand::from_str("QQQAA 483").unwrap(), HandType::FullHouse)]
    #[case(Hand::from_str("KKKKK 0").unwrap(), HandType::FiveOfAKind)]
    fn test_detect_hand_type(#[case] hand: Hand, #[case] expected: HandType) {
        let hand_type = HandType::from(&hand);
        assert_eq!(hand_type, expected);
    }

    #[rstest]
    #[case(Hand::from_str("32T3K 765").unwrap(), Hand::from_str("T55J5 684").unwrap(), Right)]
    #[case(Hand::from_str("KK677 0").unwrap(), Hand::from_str("KTJJT 0").unwrap(), Left)]
    #[case(Hand::from_str("77888 0").unwrap(), Hand::from_str("77788 0").unwrap(), Left)]
    #[case(Hand::from_str("T55J5 0").unwrap(), Hand::from_str("QQQJA 0").unwrap(), Right)]
    #[case(Hand::from_str("KTJJT 220").unwrap(), Hand::from_str("T55J5 684").unwrap(), Right)]
    #[case(Hand::from_str("JJJJJ 417").unwrap(), Hand::from_str("AAAA2 196").unwrap(), Left)]
    fn test_compare_hands(#[case] hand_l: Hand, #[case] hand_r: Hand, #[case] side: SideIsGreater) {
        match side {
            Left => assert!(hand_l > hand_r),
            Right => assert!(hand_r > hand_l),
            Equal => assert_eq!(hand_l, hand_r),
        }
    }

    #[test]
    fn test_map_jacks_to_jokers() {
        let hand = Hand::from_str("KTJJT 220").unwrap();
        let expected = Hand {
            cards: [King, Number(10), Joker, Joker, Number(10)],
            bid: 220,
        };
        let hand = hand.map_jacks_to_jokers();
        assert_eq!(hand, expected);
    }

    #[rstest]
    #[case(Hand::from_str("32T3K 765").unwrap(), HandType::OnePair)]
    #[case(Hand::from_str("T55J5 684").unwrap(), HandType::FourOfAKind)]
    #[case(Hand::from_str("KK677 0").unwrap(), HandType::TwoPairs)]
    #[case(Hand::from_str("QQQJA 483").unwrap(), HandType::FourOfAKind)]
    #[case(Hand::from_str("JJJJJ 24").unwrap(), HandType::FiveOfAKind)]
    #[case(Hand::from_str("KJKA2 0").unwrap(), HandType::ThreeOfAKind)]
    #[case(Hand::from_str("KKJAA 0").unwrap(), HandType::FullHouse)]
    #[case(Hand::from_str("KTJJT 0").unwrap(), HandType::FourOfAKind)]
    fn test_get_hand_type_with_jokers(#[case] hand: Hand, #[case] expected: HandType) {
        let hand = hand.map_jacks_to_jokers();
        let hand_type = HandType::from(&hand);
        assert_eq!(hand_type, expected);
    }

    #[rstest]
    #[case(Hand::from_str("32T3K 765").unwrap(), Hand::from_str("T55J5 684").unwrap(), Right)]
    #[case(Hand::from_str("JKKK2 1").unwrap(), Hand::from_str("2KKKK 1").unwrap(), Right)]
    #[case(Hand::from_str("KTJJT 220").unwrap(), Hand::from_str("T55J5 684").unwrap(), Left)]
    #[case(Hand::from_str("KTJJT 220").unwrap(), Hand::from_str("QQQJA 483").unwrap(), Left)]
    fn test_compare_hands_with_jokers(
        #[case] hand_l: Hand,
        #[case] hand_r: Hand,
        #[case] side: SideIsGreater,
    ) {
        let hand_l = hand_l.map_jacks_to_jokers();
        let hand_r = hand_r.map_jacks_to_jokers();
        match side {
            Left => assert!(
                hand_l > hand_r,
                "Left: {:?}, Right: {:?}",
                HandType::from(&hand_l),
                HandType::from(&hand_r)
            ),
            Right => assert!(hand_r > hand_l),
            Equal => assert_eq!(hand_l, hand_r),
        }
    }
}
