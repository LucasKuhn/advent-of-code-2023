fn main() {
    let input = include_str!("../../input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut start: (usize, usize) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        matrix.push(Vec::new());
        for (j, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (i, j);
            }
            matrix[i].push(char);
        }
    }

    println!("START: {:?}", start);
    for line in &matrix {
        println!("{:?}", line);
    }

    let (x, y) = start;
    let mut next_location: (usize, usize) = (0, 0);
    if matrix[x + 1][y] == '|' {
        next_location = (x + 1, y);
    }
    if matrix[x - 1][y] == '|' {
        next_location = (x - 1, y);
    }
    if matrix[x][y + 1] == '-' {
        next_location = (x, y + 1);
    }
    if matrix[x][y - 1] == '-' {
        next_location = (x, y - 1);
    }
    println!("Next location: {:?}", next_location);
}
