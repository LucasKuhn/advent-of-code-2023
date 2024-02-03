fn main() {
    let input = include_str!("../../input.txt");

    let mut universe: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        universe.push(line.chars().collect());
    }

    // -- Duplicating empty lines

    let mut empty_lines_location: Vec<i32> = Vec::new();
    for (i, line) in universe.iter().enumerate() {
        if !line.contains(&'#') {
            empty_lines_location.push(i as i32);
        }
    }

    let line_length = universe.first().unwrap().len();
    for line in empty_lines_location.iter().rev() {
        universe.insert(*line as usize, vec!['.'; line_length]);
    }

    // -- Duplicating empty columns

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

    for line in &mut universe {
        for col in empty_columns_location.iter().rev() {
            line.insert(*col as usize, '.');
        }
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
            let distance_x = (*k - *i).abs();
            let distance_y = (*l - *j).abs();
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
