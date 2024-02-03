use std::collections::HashMap;

fn is_symbol(char: char) -> bool {
    char.is_ascii_punctuation() && char != '.'
}

fn has_adjacent_symbol(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if is_symbol(matrix[y + 1][x]) {
        return true;
    }
    if is_symbol(matrix[y - 1][x]) {
        return true;
    }
    if is_symbol(matrix[y][x + 1]) {
        return true;
    }
    if is_symbol(matrix[y][x - 1]) {
        return true;
    }
    if is_symbol(matrix[y + 1][x + 1]) {
        return true;
    }
    if is_symbol(matrix[y + 1][x - 1]) {
        return true;
    }
    if is_symbol(matrix[y - 1][x + 1]) {
        return true;
    }
    if is_symbol(matrix[y - 1][x - 1]) {
        return true;
    }
    false
}

fn main() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    // let input = include_str!("../../input.txt");
    let line_length = input.find('\n').unwrap() + 2;

    let mut matrix: Vec<Vec<char>> = Vec::new();

    matrix.push(vec!['.'; line_length]);
    for line in input.lines() {
        matrix.push(format!(".{}.", line).chars().collect());
    }
    matrix.push(vec!['.'; line_length]);

    for line in &matrix {
        println!("{:?}", line);
    }

    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    dbg!(&gears);

    let width = matrix[0].len();
    let height = matrix.len();

    let mut x = 0;
    let mut y = 0;
    while y < height {
        while x < width {
            let char = matrix[y][x];
            if char == '*' {
                println!("Found gear at ({}, {})", x, y);
                gears.insert((x, y), vec![]);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    dbg!(&gears);
}
