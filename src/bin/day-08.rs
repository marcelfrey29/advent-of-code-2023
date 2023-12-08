use advent_of_code_2023::get_lines_from_file;
use std::{
    collections::{HashMap, VecDeque},
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_steps_to_goal(lines);

    println!("Result: {}", result)
}

type LocationMap = HashMap<String, (String, String)>;

fn get_steps_to_goal(lines: Vec<String>) -> i64 {
    let (pattern, map) = parse_map(lines);
    get_steps(pattern, map)
}

fn parse_map(lines: Vec<String>) -> (String, LocationMap) {
    let mut lines: VecDeque<String> = VecDeque::from(lines);
    let mut map: LocationMap = HashMap::new();
    let pattern = lines.pop_front().expect("");
    lines.pop_front(); // Remove empty line between pattern and map data

    for line in lines {
        // AAA = (BBB, CCC)
        let line = line
            .replace(" = ", " ")
            .replace('(', "")
            .replace(", ", " ")
            .replace(')', "");
        let parts = line.split(' ').collect::<Vec<&str>>();
        map.insert(
            parts.first().expect("Expected from part").to_string(),
            (
                parts.get(1).expect("Expected to part for Left").to_string(),
                parts
                    .get(2)
                    .expect("Expected to part for Right")
                    .to_string(),
            ),
        );
    }

    (pattern, map)
}

fn get_steps(pattern: String, map: LocationMap) -> i64 {
    let pattern = pattern.chars();
    let mut pattern_position = 0;
    let mut counter: i64 = 0;
    let mut current_location: String = String::from("AAA"); // Start Location

    while current_location != *"ZZZ" {
        let current_node = map
            .get(&current_location)
            .expect("Current Location must exist in map.");

        let next_step = pattern.clone().nth(pattern_position).expect("");
        pattern_position += 1;
        if pattern_position >= pattern.clone().count() {
            pattern_position = 0;
        }

        if next_step == 'L' {
            current_location = current_node.0.clone();
        } else {
            current_location = current_node.1.clone();
        }

        counter += 1;
    }

    counter
}

#[test]
fn test_get_steps() {
    let map_data = vec![
        ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
        ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
        ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
    ];
    let map: LocationMap = map_data.into_iter().collect();
    assert_eq!(get_steps("LLR".to_string(), map), 6);
}

#[test]
fn test_get_steps_to_goal() {
    assert_eq!(
        get_steps_to_goal(vec![
            "LLR".to_string(),
            "".to_string(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]),
        6
    );
    assert_eq!(
        get_steps_to_goal(vec![
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]),
        2
    )
}
