fn main() {
    let codes: Vec<&str> = include_str!("../../input.txt").lines().collect();

    let mut digits_string_list: Vec<String> = vec![];

    for code in &codes {
        let mut digit = String::new();
        for c in code.chars() {
            if c.is_digit(10) {
                digit.push(c);
                break;
            }
        }
        for c in code.chars().rev() {
            if c.is_digit(10) {
                digit.push(c);
                break;
            }
        }
        digits_string_list.push(digit.clone());
    }

    let digits_list: Vec<i32> = digits_string_list
        .iter()
        .map(|str| str.parse().unwrap_or_default())
        .collect();

    let total: i32 = digits_list.iter().sum();

    println!("TOTAL: {}", total)
}
