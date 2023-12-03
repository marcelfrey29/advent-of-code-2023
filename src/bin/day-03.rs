use std::{env, vec};

use advent_of_code_2023::get_lines_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_part_number_sum(lines);

    println!("Result: {}", result)
}

fn get_part_number_sum(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for i in 0..lines.len() {
        let current_line = lines.get(i).expect("");
        let potential_part_numbers_in_current_line = get_part_numbers(current_line);
        for potential_part_number in potential_part_numbers_in_current_line {
            if part_number_has_adjacent_symbol(
                potential_part_number.clone(),
                if i > 0 { lines.get(i - 1) } else { None },
                current_line,
                lines.get(i + 1),
            ) {
                sum += potential_part_number.number
            }
        }
    }

    sum
}

fn get_part_numbers(line: &str) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut part_number_active = false;
    let mut current_number = String::from("0");
    let mut part_number: PartNumber = PartNumber {
        number: 0,
        start_index: 0,
        end_index: 0,
    };

    for (current_index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            if part_number_active {
                current_number = format!("{current_number}{char}");
            } else {
                part_number_active = true;
                part_number.start_index = current_index;
                current_number = format!("{char}");
            }
        } else {
            // We don't want to create PartNumbers when there is no active number
            // e.g. for ".........."
            if part_number_active {
                // Create and save Part Number object
                part_number.end_index = if current_index > 0 {
                    current_index - 1
                } else {
                    0
                };
                part_number.number = current_number.parse::<i32>().expect("Should be a number.");
                part_numbers.push(part_number);
                part_number_active = false;
                // Create new PartNumber object
                part_number = PartNumber {
                    number: 0,
                    start_index: 0,
                    end_index: 0,
                }
            }
        }
    }

    if part_number_active {
        // Create and save Part Number object
        part_number.end_index = line.len() - 1;
        part_number.number = current_number.parse::<i32>().expect("Should be a number.");
        part_numbers.push(part_number);
    }

    part_numbers
}

fn part_number_has_adjacent_symbol(
    part_number: PartNumber,
    previous_line: Option<&String>,
    current_line: &str,
    next_line: Option<&String>,
) -> bool {
    // Previous Line
    if previous_line.is_some() {
        let start = if part_number.start_index == 0 {
            0
        } else {
            part_number.start_index - 1
        };
        for i in start..=part_number.end_index + 1 {
            let char_at_index = previous_line.unwrap().chars().nth(i).unwrap_or('.');
            if !char_at_index.is_numeric() && char_at_index != '.' {
                return true;
            }
        }
    }
    // Current Line
    let left_char = current_line
        .chars()
        .nth(if part_number.start_index > 0 {
            part_number.start_index - 1
        } else {
            usize::MAX // Prevent Panix if value is less than zero, make sure `NONE` is returned which should be the case with the MAX value
        })
        .unwrap_or('.');
    let right_char = current_line
        .chars()
        .nth(part_number.end_index + 1)
        .unwrap_or('.');
    if (!left_char.is_numeric() && left_char != '.')
        || (!right_char.is_numeric() && right_char != '.')
    {
        return true;
    }

    // Next Line
    if next_line.is_some() {
        let start = if part_number.start_index == 0 {
            0
        } else {
            part_number.start_index - 1
        };
        for i in start..=part_number.end_index + 1 {
            let char_at_index = next_line.unwrap().chars().nth(i).unwrap_or('.');
            if !char_at_index.is_numeric() && char_at_index != '.' {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, PartialEq, Clone)]
struct PartNumber {
    number: i32,
    start_index: usize,
    end_index: usize,
}

#[test]
fn test_get_part_numbers() {
    assert_eq!(get_part_numbers(""), vec![]);
    assert_eq!(get_part_numbers(".........."), vec![]);
    assert_eq!(get_part_numbers(".*..$.#.+."), vec![]);
    assert_eq!(get_part_numbers(".........."), vec![]);
    assert_eq!(
        get_part_numbers("1........."),
        vec![PartNumber {
            number: 1,
            start_index: 0,
            end_index: 0
        }]
    );
    assert_eq!(
        get_part_numbers(".........1"),
        vec![PartNumber {
            number: 1,
            start_index: 9,
            end_index: 9
        }]
    );
    assert_eq!(
        get_part_numbers("...1......"),
        vec![PartNumber {
            number: 1,
            start_index: 3,
            end_index: 3
        }]
    );
    assert_eq!(
        get_part_numbers("...10....."),
        vec![PartNumber {
            number: 10,
            start_index: 3,
            end_index: 4
        }]
    );
    assert_eq!(
        get_part_numbers("1.23..678."),
        vec![
            PartNumber {
                number: 1,
                start_index: 0,
                end_index: 0
            },
            PartNumber {
                number: 23,
                start_index: 2,
                end_index: 3
            },
            PartNumber {
                number: 678,
                start_index: 6,
                end_index: 8
            }
        ]
    );
    assert_eq!(
        get_part_numbers("1000000000"),
        vec![PartNumber {
            number: 1000000000,
            start_index: 0,
            end_index: 9
        }]
    );
}

#[test]
fn test_get_part_number_sum() {
    assert_eq!(get_part_number_sum(vec![String::from("")]), 0);
    assert_eq!(get_part_number_sum(vec![String::from("*1")]), 1);
    assert_eq!(get_part_number_sum(vec![String::from("1*")]), 1);
    assert_eq!(
        get_part_number_sum(vec![
            String::from("*.."),
            String::from("1.."),
            String::from("...")
        ]),
        1
    );
    assert_eq!(
        get_part_number_sum(vec![
            String::from("..."),
            String::from("1.."),
            String::from("*..")
        ]),
        1
    );
    assert_eq!(
        get_part_number_sum(vec![
            String::from("*.."),
            String::from(".1."),
            String::from("...")
        ]),
        1
    );
    assert_eq!(
        get_part_number_sum(vec![
            String::from("..*"),
            String::from(".1."),
            String::from("...")
        ]),
        1
    );
    assert_eq!(
        get_part_number_sum(vec![
            String::from("..."),
            String::from(".1."),
            String::from("*..")
        ]),
        1
    );
    assert_eq!(
        get_part_number_sum(vec![
            String::from("..."),
            String::from(".1."),
            String::from("..*")
        ]),
        1
    );
    assert_eq!(
        get_part_number_sum(vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ]),
        4361
    );
}
