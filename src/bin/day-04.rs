use std::env;

use advent_of_code_2023::get_lines_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_winning_card_number_sum(lines);

    println!("Result: {}", result)
}

fn get_winning_card_number_sum(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    for line in lines {
        println!("Line: {}", line);
        sum += get_card_value(&line);
    }

    sum
}

fn get_card_value(line: &str) -> i32 {
    println!("Card: {}", line);
    let number_part_str: &str = line
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Expected right side of card to be available.");
    println!("Card Number-Part: {}", number_part_str);
    let mut sum = 0;
    let parts: Vec<&str> = number_part_str.split('|').collect();
    let winning_part = get_numbers_from_card_part(parts.clone(), 0);
    let number_part = get_numbers_from_card_part(parts, 1);

    for number in number_part {
        if winning_part.contains(&number) {
            if sum == 0 {
                sum = 1;
            } else {
                sum *= 2
            }
        }
    }
    sum
}

fn get_numbers_from_card_part(part: Vec<&str>, index: i32) -> Vec<i32> {
    println!("Get Numbers from Card: {:?} at index {}.", part, index);
    let part: Vec<i32> = part
        .get(index as usize)
        .unwrap_or_else(|| panic!("Expected part at index {} to be available", index))
        .trim()
        .replace("  ", " ")
        .split(' ')
        .map(|x| x.trim().parse::<i32>().expect("Expected a Number"))
        .collect();
    part
}

#[test]
fn test_get_numbers_from_card_part() {
    assert_eq!(
        get_numbers_from_card_part(vec!["1 15 3", ""], 0),
        vec![1, 15, 3]
    );
    assert_eq!(
        get_numbers_from_card_part(vec!["", "1 15 3"], 1),
        vec![1, 15, 3]
    );
}

#[test]
fn test_get_card_value() {
    assert_eq!(get_card_value("Card 1: 1 | 1"), 1);
    assert_eq!(get_card_value("Card 1: 1 2 | 1 2 3 4"), 2);
    assert_eq!(get_card_value("Card 1: 1 2 3 | 1 2 3"), 4);
    assert_eq!(get_card_value("Card 1: 1 2 3 | 4 5 6"), 0);
}

#[test]
fn test_get_winning_card_number_sum() {
    assert_eq!(get_winning_card_number_sum(vec![]), 0);
    assert_eq!(
        get_winning_card_number_sum(vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ]),
        13
    );
}
