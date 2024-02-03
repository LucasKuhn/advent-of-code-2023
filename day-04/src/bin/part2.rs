// DRAFT

#[derive(Debug, Default)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
    matches: usize,
    points: usize,
}

impl From<&str> for Card {
    /// Parse a string into a card
    /// ### Input example:
    /// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    fn from(raw: &str) -> Self {
        let (card_number, info) = raw.split_once(':').unwrap();
        let (raw_winning_numbers, raw_numbers) = info.split_once('|').unwrap();

        let id: usize = card_number
            .split(' ')
            .filter(|&x| !x.is_empty())
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let numbers: Vec<usize> = raw_numbers
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let winning_numbers: Vec<usize> = raw_winning_numbers
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim().parse().unwrap())
            .collect();

        Card {
            id,
            winning_numbers,
            numbers,
            ..Card::default()
        }
    }
}

impl Card {
    fn count_matches(&mut self) {
        let mut matches = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                matches += 1;
            }
        }
        self.matches = matches;
    }

    fn calculate_points(&mut self) {
        let matches: u32 = self.matches as u32;
        if matches > 0 {
            let base: u32 = 2;
            let points = base.pow(matches - 1);
            self.points = points as usize;
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let card = Card::from(line);
        cards.push(card);
    }

    for card in &mut cards {
        card.count_matches();
        card.calculate_points();
    }

    let total: usize = cards.iter().map(|card| card.points).sum();

    println!("TOTAL: {}", total);
}
