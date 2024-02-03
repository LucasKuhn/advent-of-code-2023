use itertools::Itertools;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<char>,
    pub bet: usize,
    pub hand_type: HandType,
    pub hand_value: usize,
}

impl Hand {
    pub fn new(cards: &str, bet: &str) -> Self {
        let cards: Vec<char> = cards.chars().collect();
        let bet = bet.parse().unwrap();
        let hand_type = Hand::get_hand_type(&cards).unwrap();
        let hand_value = Hand::get_hand_value(&cards, &hand_type).unwrap();

        Hand {
            cards,
            bet,
            hand_type,
            hand_value,
        }
    }

    fn get_hand_type(cards: &[char]) -> Option<HandType> {
        let frequency = cards.iter().counts();
        let mut iter = frequency.values().sorted().rev().cloned();
        let highest_count = iter.next().unwrap_or(0);
        let second_highest_count = iter.next().unwrap_or(0);

        if highest_count == 3 && second_highest_count == 2 {
            return Some(HandType::FullHouse);
        }
        if highest_count == 2 && second_highest_count == 2 {
            return Some(HandType::TwoPairs);
        }

        match highest_count {
            5 => Some(HandType::FiveOfAKind),
            4 => Some(HandType::FourOfAKind),
            3 => Some(HandType::ThreeOfAKind),
            2 => Some(HandType::OnePair),
            _ => Some(HandType::HighCard),
        }
    }

    fn get_hand_value(cards: &[char], hand_type: &HandType) -> Option<usize> {
        let cards_order = "023456789TJQKA";
        let mut hand_value = format!("{:02}", *hand_type as usize);
        for card in cards {
            let card_value = cards_order.find(*card).unwrap();
            hand_value.push_str(&format!("{:02}", card_value));
        }
        Some(hand_value.parse().unwrap())
    }
}
