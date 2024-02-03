// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4
fn main() {
    let input = include_str!("../../input.txt");

    for line in input.lines() {
        println!("{}", line);
    }

    let mut input = input.split("\n\n");
    let seeds_input = input.next();
    let seed_to_soil_input = input.next();
    let soil_to_fertilizer_input = input.next();
    let fertilizer_to_water_input = input.next();
    let water_to_light_input = input.next();
    let light_to_temperature_input = input.next();
    let temperature_to_humidity_input = input.next();
    let humidity_to_location_input = input.next();

    dbg!(&seeds_input);
    dbg!(&seed_to_soil_input);
    dbg!(&soil_to_fertilizer_input);
    dbg!(&fertilizer_to_water_input);
    dbg!(&water_to_light_input);
    dbg!(&light_to_temperature_input);
    dbg!(&temperature_to_humidity_input);
    dbg!(&humidity_to_location_input);
}
