use std::env;

use advent_of_code_2023::get_lines_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_game_id_sum(lines);

    println!("Result Part 1: {}, Result Part 2: {}", result.0, result.1)
}

fn get_game_id_sum(lines: Vec<String>) -> (i32, i32) {
    let mut part_one_result = 0;
    let mut part_two_result = 0;

    for line in lines {
        let game: Game = line.into();
        let max_cubes_per_color = game.clone().get_max_cube_counts();
        if max_cubes_per_color.red <= 12
            && max_cubes_per_color.green <= 13
            && max_cubes_per_color.blue <= 14
        {
            part_one_result += game.number
        }
        part_two_result +=
            max_cubes_per_color.red * max_cubes_per_color.green * max_cubes_per_color.blue
    }

    (part_one_result, part_two_result)
}

/// Represents a single game.
/// Every game has an ID and a list of cubes that are revealed.
#[derive(Debug, Clone)]
struct Game {
    number: i32,
    game_records: Vec<RevealedCubes>,
}

impl Game {
    fn get_max_cube_counts(self) -> RevealedCubes {
        let mut cube_result: RevealedCubes = RevealedCubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in self.game_records {
            if cube.red > cube_result.red {
                cube_result.red = cube.red
            }
            if cube.green > cube_result.green {
                cube_result.green = cube.green
            }
            if cube.blue > cube_result.blue {
                cube_result.blue = cube.blue
            }
        }
        println!("Max. Cube count: {:?}", cube_result);
        cube_result
    }
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        println!("Game: {value}");
        let parts: Vec<&str> = value.split(':').collect();
        let game = &parts
            .first()
            .expect("Game should have an identifier part.")
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .expect("Game should have an record part.")
            .to_string()
            .parse::<i32>()
            .expect("Game ID should be a number.");

        let mut records: Vec<RevealedCubes> = vec![];
        for record in parts.last().expect("").split(';').collect::<Vec<&str>>() {
            records.push(record.to_string().into())
        }

        Game {
            number: game.to_owned(),
            game_records: records,
        }
    }
}

/// The cubes that are revelead in a single subset
#[derive(Debug, Clone)]
struct RevealedCubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl From<String> for RevealedCubes {
    fn from(value: String) -> Self {
        println!("Revealed Cubes: {value}");
        let mut result = RevealedCubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube_details in value.split(',').collect::<Vec<&str>>() {
            // Get the cube which color and amount...
            let cube: Cube = cube_details.to_string().into();
            // ... and set the amount based on the color.
            match cube.color.as_str() {
                "red" => result.red = cube.count,
                "green" => result.green = cube.count,
                "blue" => result.blue = cube.count,
                _ => {
                    println!("ERROR: Received an invalid color.")
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
struct Cube {
    color: String,
    count: i32,
}

impl From<String> for Cube {
    fn from(value: String) -> Self {
        println!("Cube Details: {value}");
        let parts = value.trim().split(' ').collect::<Vec<&str>>();
        Cube {
            color: parts.last().expect("Color expected").to_string(),
            count: parts
                .first()
                .expect("Count expected")
                .parse::<i32>()
                .unwrap(),
        }
    }
}

#[test]
fn test_get_game_id_sum() {
    //  assert_eq!(get_game_id_sum(vec![]), 0);
    assert_eq!(
        get_game_id_sum(vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ]),
        (8, 2286)
    );
}
