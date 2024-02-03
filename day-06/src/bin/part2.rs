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

    let time_string: String = raw_times.split_whitespace().collect();
    let distance_string: String = raw_distances.split_whitespace().collect();

    let mut race = Race {
        time_limit: time_string.parse().unwrap(),
        distance_record: distance_string.parse().unwrap(),
        ways_to_beat: 0,
    };

    race.simulate();

    println!("TOTAL: {}", race.ways_to_beat);
}
