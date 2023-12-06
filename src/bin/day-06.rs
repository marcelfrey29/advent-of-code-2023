use std::{env, vec};

use advent_of_code_2023::get_lines_from_file;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_ways_to_beat_the_record_result(lines);

    println!("Result: {}", result)
}

fn get_ways_to_beat_the_record_result(lines: Vec<String>) -> i32 {
    let races = get_races(lines);
    let mut result = 1; // Safe default for multiplication, might cause problems if we can't win a race at all
    for race in races {
        result *= race.get_possible_combinations_to_win()
    }
    result
}

fn get_races(data: Vec<String>) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let remove_multiple_spaces = Regex::new(r" +").unwrap();
    let times = remove_multiple_spaces
        .replace_all(data.first().expect("Expected Time Line"), " ")
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("")
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>().expect("Expected Number."))
        .collect::<Vec<i32>>();
    let distances = remove_multiple_spaces
        .replace_all(data.get(1).expect("Expected Time Line"), " ")
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("")
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>().expect("Expected Number."))
        .collect::<Vec<i32>>();

    for i in 0..times.len() {
        races.push(Race {
            time: *times.get(i).expect(""),
            distance: *distances.get(i).expect(""),
        })
    }

    races
}

#[derive(Debug, Clone, PartialEq)]
struct Race {
    time: i32,
    distance: i32,
}

impl Race {
    fn get_possible_combinations_to_win(self) -> i32 {
        let mut possible_combinations_to_win = 0;
        for button_hold in 0..self.time {
            let travel_distance = button_hold * (self.time - button_hold);
            if travel_distance > self.distance {
                possible_combinations_to_win += 1
            }
        }
        possible_combinations_to_win
    }
}

#[test]
fn test_get_race() {
    assert_eq!(
        get_races(vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string()
        ]),
        vec![
            Race {
                time: 7,
                distance: 9
            },
            Race {
                time: 15,
                distance: 40
            },
            Race {
                time: 30,
                distance: 200
            }
        ]
    )
}

#[test]
fn test_get_possible_combinations_to_win_from_race() {
    assert_eq!(
        Race {
            time: 7,
            distance: 9
        }
        .get_possible_combinations_to_win(),
        4
    )
}

#[test]
fn test_get_ways_to_beat_the_record_result() {
    assert_eq!(
        get_ways_to_beat_the_record_result(vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string()
        ]),
        288
    )
}
