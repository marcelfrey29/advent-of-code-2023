use std::{
    collections::{HashMap, VecDeque},
    env, vec,
};

use advent_of_code_2023::get_lines_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_lowest_location_number(lines);

    println!("Result: {}", result)
}

fn get_lowest_location_number(lines: Vec<String>) -> i64 {
    let mut lines: VecDeque<String> = VecDeque::from(lines);
    let mut lowest_location_number = i64::MAX;

    let seeds = get_seeds(&lines.pop_front().expect("Expeced seed line."));
    lines
        .pop_front()
        .expect("Expected empty line after seed line.");

    let (seed_to_soil, rest) = parse_map(lines);
    let (soild_to_fertilizer, rest) = parse_map(rest);
    let (fertilizer_to_water, rest) = parse_map(rest);
    let (water_to_light, rest) = parse_map(rest);
    let (light_to_temperature, rest) = parse_map(rest);
    let (temperature_to_humidity, rest) = parse_map(rest);
    let (humidity_to_location, _) = parse_map(rest);

    let mut directions: Vec<i64> = vec![];
    for seed in seeds {
        let next = *seed_to_soil.get(&seed).unwrap_or(&seed);
        let next = *soild_to_fertilizer.get(&next).unwrap_or(&next);
        let next = *fertilizer_to_water.get(&next).unwrap_or(&next);
        let next = *water_to_light.get(&next).unwrap_or(&next);
        let next = *light_to_temperature.get(&next).unwrap_or(&next);
        let next = *temperature_to_humidity.get(&next).unwrap_or(&next);
        let next = *humidity_to_location.get(&next).unwrap_or(&next);
        directions.push(next);
    }

    for direction in directions {
        if direction < lowest_location_number {
            lowest_location_number = direction
        }
    }

    lowest_location_number
}

fn get_seeds(line: &str) -> Vec<i64> {
    line.split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Seed line should contain a seed part.")
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|seed| seed.trim())
        .map(|seed| seed.parse::<i64>().expect("Seed should be a number."))
        .collect::<Vec<i64>>()
}

fn parse_map(mut map_data: VecDeque<String>) -> (HashMap<i64, i64>, VecDeque<String>) {
    let mut map: HashMap<i64, i64> = HashMap::new();
    let title = map_data.pop_front().expect("Expected map title"); // Remove title line
    println!("Starting to generate map '{title}'");
    loop {
        let line = map_data.pop_front().unwrap_or_default();
        // End processing on empty line
        if line.is_empty() {
            println!("Generated Map '{title}': {:?}", map);
            return (map, map_data);
        }

        // Create Map
        let mapping_parts = line
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().expect("Mapping should be a number"))
            .collect::<Vec<i64>>();
        let destination_range_start = *mapping_parts.first().expect("");
        let source_range_start = *mapping_parts.get(1).expect("");
        let range = *mapping_parts.get(2).expect("");

        for x in 0..range {
            println!("Processing map '{title}': {x}");
            map.insert(source_range_start + x, destination_range_start + x);
        }
    }
}

#[test]
fn test_get_seeds() {
    assert_eq!(get_seeds("seeds: 79 14 55 13"), vec![79, 14, 55, 13]);
}

#[test]
fn test_parse_map() {
    let mut result_map: HashMap<i64, i64> = HashMap::new();
    result_map.insert(98, 50);
    result_map.insert(99, 51);
    result_map.insert(50, 52);
    result_map.insert(51, 53);
    result_map.insert(52, 54);
    result_map.insert(53, 55);
    assert_eq!(
        parse_map(VecDeque::from(vec![
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 4".to_string(), // Different from Example, because smaller ;)
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
        ])),
        (
            result_map,
            VecDeque::from(vec![
                "soil-to-fertilizer map:".to_string(),
                "0 15 37".to_string(),
                "37 52 2".to_string(),
                "39 0 15".to_string(),
                "".to_string(),
                "fertilizer-to-water map:".to_string(),
                "49 53 8".to_string(),
                "0 11 42".to_string(),
                "42 0 7".to_string(),
                "57 7 4".to_string(),
                "".to_string(),
                "water-to-light map:".to_string(),
                "88 18 7".to_string(),
                "18 25 70".to_string(),
                "".to_string(),
                "light-to-temperature map:".to_string(),
                "45 77 23".to_string(),
                "81 45 19".to_string(),
                "68 64 13".to_string(),
            ])
        )
    );
}

#[test]
fn test_get_lowest_location_number() {
    assert_eq!(
        get_lowest_location_number(vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
            "".to_string(),
        ]),
        35
    )
}
