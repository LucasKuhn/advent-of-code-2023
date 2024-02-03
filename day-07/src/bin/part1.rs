use day_07::Hand;

fn main() {
    let input = include_str!("../../input.txt");
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let (cards, bet) = line.split_once(' ').unwrap();
        let hand = Hand::new(cards, bet);
        hands.push(hand);
    }

    hands.sort_by_key(|hand| hand.hand_value);

    let mut total_winnings = 0;
    for (i, card) in hands.iter().enumerate() {
        total_winnings += card.bet * (i + 1);
    }
    println!("Total winnings: {}", total_winnings);
}
