use advent_of_code_2023::get_lines_from_file;
use std::{env, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = get_lines_from_file(file_path);
    let result = get_sum_of_extrapolated_values(lines);

    println!("Result: {}", result)
}

fn get_sum_of_extrapolated_values(lines: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for line in lines {
        sum += process_lines(line);
    }

    sum
}

fn process_lines(lines: String) -> i32 {
    let values: Vec<i32> = lines
        .split(' ')
        .map(|x| x.parse::<i32>().expect("Expected Number in Sequence"))
        .collect();

    get_next_value_for_sequence(values)
}

fn get_next_value_for_sequence(sequence: Vec<i32>) -> i32 {
    println!("Processing Sequence: {:?}", sequence);

    if sequence.iter().all(|x| x == &0) {
        return 0;
    }

    let mut differences: Vec<i32> = vec![];

    for i in 1..sequence.len() {
        let a = sequence.get(i - 1).expect("Element at i-1 should exist.");
        let b = sequence.get(i).expect("Element at i should exist. ");
        let difference = b - a;
        println!(
            "The difference for '{}' and '{}' is '{}'.",
            a, b, difference
        );
        differences.push(b - a)
    }

    let difference_to_apply = get_next_value_for_sequence(differences);
    println!("The difference to apply is '{}'.", difference_to_apply);
    return sequence.last().expect("Last element should exist.") + difference_to_apply;
}

#[test]
fn test_get_next_value_for_sequence() {
    assert_eq!(get_next_value_for_sequence(vec![1, 3, 6, 10, 15, 21]), 28);
}

#[test]
fn test_process_line() {
    assert_eq!(process_lines("0 0".to_string()), 0);
    assert_eq!(process_lines("2 2 2".to_string()), 2);
    assert_eq!(process_lines("10 13 16 21 30 45".to_string()), 68);
}

#[test]
fn test_get_sum_of_extrapolated_values() {
    assert_eq!(
        get_sum_of_extrapolated_values(vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string()
        ]),
        114
    );
}
