#[derive(Debug, Default)]
struct Game {
    id: usize,
    valid: bool,
    red_throws: Vec<usize>,
    green_throws: Vec<usize>,
    blue_throws: Vec<usize>,
}

fn main() {
    let mut games: Vec<Game> = vec![];
    let inputs = include_str!("../../part1-input").lines();

    for (i, input) in inputs.enumerate() {
        let input_without_game = input.split(':').nth(1).unwrap();

        let throws: Vec<_> = input_without_game
            .split([',', ';'])
            .map(|x| x.trim())
            .collect();

        let mut game = Game {
            id: i + 1,
            valid: true,
            ..Game::default()
        };

        for throw in &throws {
            let mut iter = throw.split(' ');
            let amount = iter.next().unwrap();
            let color_name = iter.next().unwrap();
            match color_name {
                "red" => game.red_throws.push(amount.parse().unwrap()),
                "green" => game.green_throws.push(amount.parse().unwrap()),
                "blue" => game.blue_throws.push(amount.parse().unwrap()),
                _ => panic!("Unknown color: {}", color_name),
            }
        }
        games.push(game);
    }

    for game in &mut games {
        let biggest_red = *game.red_throws.iter().max().unwrap();
        let biggest_green = *game.green_throws.iter().max().unwrap();
        let biggest_blue = *game.blue_throws.iter().max().unwrap();

        if biggest_red > 12 {
            game.valid = false;
        }
        if biggest_green > 13 {
            game.valid = false;
        }
        if biggest_blue > 14 {
            game.valid = false;
        }
    }

    let valid_games: Vec<Game> = games.into_iter().filter(|game| game.valid).collect();

    let total: usize = valid_games.into_iter().map(|game| game.id).sum();
    println!("TOTAL: {}", total)
}
