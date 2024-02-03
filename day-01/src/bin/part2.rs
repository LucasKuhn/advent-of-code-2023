fn get_first_number(input: &str) -> char {
    for i in 0..input.len() {
        let slice = &input[i..];

        let first_char = slice.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            return first_char;
        }

        match slice {
            _ if slice.starts_with("one") => return '1',
            _ if slice.starts_with("two") => return '2',
            _ if slice.starts_with("three") => return '3',
            _ if slice.starts_with("four") => return '4',
            _ if slice.starts_with("five") => return '5',
            _ if slice.starts_with("six") => return '6',
            _ if slice.starts_with("seven") => return '7',
            _ if slice.starts_with("eight") => return '8',
            _ if slice.starts_with("nine") => return '9',
            _ => continue,
        };
    }
    '0'
}

fn get_last_number(input: &str) -> char {
    for i in (1..input.len() + 1).rev() {
        let slice = &input[..i];

        let last_char = slice.chars().last().unwrap();
        if last_char.is_ascii_digit() {
            return last_char;
        }

        match slice {
            _ if slice.ends_with("one") => return '1',
            _ if slice.ends_with("two") => return '2',
            _ if slice.ends_with("three") => return '3',
            _ if slice.ends_with("four") => return '4',
            _ if slice.ends_with("five") => return '5',
            _ if slice.ends_with("six") => return '6',
            _ if slice.ends_with("seven") => return '7',
            _ if slice.ends_with("eight") => return '8',
            _ if slice.ends_with("nine") => return '9',
            _ => continue,
        };
    }
    '0'
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut total = 0;

    for line in input.lines() {
        let tens = get_first_number(line);
        let ones = get_last_number(line);

        let number: i32 = format!("{}{}", tens, ones).parse().unwrap();
        total += number;
    }

    println!("TOTAL: {}", total);
}
