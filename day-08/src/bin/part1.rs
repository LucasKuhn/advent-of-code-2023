// RL

// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let (directions, lines) = input.split_once("\n\n").unwrap();

    let mut waypoints: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines.lines() {
        let (key, value) = line.split_once(" = ").unwrap();
        let value = value
            .trim_matches('(')
            .trim_matches(')')
            .split(", ")
            .collect();
        waypoints.insert(key, value);
    }

    let mut steps = 0;
    let mut current_waypoint = "AAA";
    'infinite_loop: loop {
        for char in directions.chars() {
            let next_waypoint = waypoints.get(current_waypoint).unwrap();
            if char == 'L' {
                current_waypoint = next_waypoint[0];
            } else if char == 'R' {
                current_waypoint = next_waypoint[1];
            }
            steps += 1;
            if current_waypoint == "ZZZ" {
                break 'infinite_loop;
            }
        }
    }
    print!("Steps: {}", steps);
}
