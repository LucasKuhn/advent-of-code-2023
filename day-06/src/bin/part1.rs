#[derive(Debug)]
struct Race {
    time_limit: usize,
    distance_record: usize,
    ways_to_beat: usize,
}

impl Race {
    fn simulate(&mut self) {
        let mut beat_count = 0;
        for time_spent in 1..self.time_limit {
            let remaining_time = self.time_limit - time_spent;
            let speed = time_spent;
            let distance = speed * remaining_time;
            if distance > self.distance_record {
                beat_count += 1;
            }
        }
        self.ways_to_beat = beat_count;
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let mut lines = input.lines();
    let raw_times = lines.next().unwrap().split("Time:").last().unwrap();
    let raw_distances = lines.next().unwrap().split("Distance:").last().unwrap();

    let times: Vec<usize> = raw_times
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<usize> = raw_distances
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut races: Vec<Race> = Vec::new();

    let iter = std::iter::zip(&times, &distances);

    for (&time, &distance) in iter {
        races.push(Race {
            time_limit: time,
            distance_record: distance,
            ways_to_beat: 0,
        })
    }

    races.iter_mut().for_each(|r| r.simulate());

    let total: usize = races.iter().map(|r| r.ways_to_beat).product();

    println!("TOTAL: {}", total)
}
