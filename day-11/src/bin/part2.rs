fn main() {
    let input = include_str!("../../input.txt");

    let mut universe: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        universe.push(line.chars().collect());
    }

    // -- Find empty lines
    let mut empty_lines_location: Vec<i32> = Vec::new();
    for (i, line) in universe.iter().enumerate() {
        if !line.contains(&'#') {
            empty_lines_location.push(i as i32);
        }
    }

    // -- Find empty columns
    let mut empty_columns_location: Vec<i32> = Vec::new();
    let line_length = universe.first().unwrap().len();
    'column_loop: for col in 0..line_length {
        for line in &universe {
            if line[col] == '#' {
                continue 'column_loop;
            }
        }
        empty_columns_location.push(col as i32);
    }

    // -- Finding galaxies
    let mut galaxies_location: Vec<(i32, i32)> = Vec::new();
    for (i, line) in universe.iter().enumerate() {
        for (j, col) in line.iter().enumerate() {
            if *col == '#' {
                galaxies_location.push((i as i32, j as i32));
            }
        }
    }

    for line in &universe {
        println!("{:?}", line);
    }
    dbg!(&empty_lines_location);
    dbg!(&empty_columns_location);

    // -- Calculating total distances
    let mut total_distances: Vec<i32> = Vec::new();
    let mut galaxies_calculated: Vec<(i32, i32)> = Vec::new();
    for (i, j) in &galaxies_location {
        for (k, l) in &galaxies_location {
            if i == k && j == l {
                continue;
            }
            if galaxies_calculated.contains(&(*k, *l)) {
                continue;
            }
            let mut distance_x = (*k - *i).abs();
            let mut distance_y = (*l - *j).abs();
            let mut increased_x = false;
            for line in &empty_lines_location {
                if *line >= *i && *line <= *k {
                    distance_x += 1;
                    increased_x = true;
                }
            }
            let mut increased_y = false;
            for col in &empty_columns_location {
                if *col >= *j && *col <= *l {
                    distance_y += 1;
                    increased_y = true;
                }
            }
            let total_distance = distance_x + distance_y;
            println!(
                "({}, {}) -> ({}, {}) = {} + {} = {}",
                i, j, k, l, distance_x, distance_y, total_distance
            );
            total_distances.push(total_distance);
        }
        galaxies_calculated.push((*i, *j));
    }

    let total: i32 = total_distances.iter().sum();
    println!("Total: {}", total);
}
