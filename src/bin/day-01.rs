use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let calibration_document = fs::read_to_string(file_path).expect("The file should exist.");
    let result = process_file(&calibration_document);
    println!("Result: {}", result)
}

/// Process the content of the file.
fn process_file(file: &str) -> i32 {
    let mut line_numbers: Vec<i32> = vec![];

    for line in file.split('\n') {
        println!("Line: {}", line);
        line_numbers.push(get_number_from_line(line))
    }

    let mut result = 0;
    for line_number in line_numbers {
        result += line_number
    }

    result
}

/// Get the two digit number from a line.
/// The number is found by combining the first and last number digit in the line.
fn get_number_from_line(line: &str) -> i32 {
    let mut numbers: Vec<char> = vec![];

    // Iterate over every character of the line and check if its numeric.
    // If it is, we push it to the numbers array.
    for char in line.chars() {
        if char.is_numeric() {
            numbers.push(char);
            println!("Found numeric charcter: {}", char)
        }
    }

    // Get the first and last element from the array, zero if no number was
    // present in the line.
    let first = numbers.first().unwrap_or(&'0');
    let last = numbers.last().unwrap_or(&'0');
    println!("Got first and last digit: {first}, {last}");

    // Combine the two numbers
    let line_number: i32 = format!("{first}{last}")
        .parse()
        .expect("Should be a number.");

    line_number
}

#[test]
fn test_process_file() {
    assert_eq!(process_file(""), 0);
    assert_eq!(process_file("a"), 0);
    assert_eq!(process_file("1"), 11);
    assert_eq!(process_file("12"), 12);
    assert_eq!(process_file("\n10"), 10);
    assert_eq!(process_file("a\n10"), 10);
    assert_eq!(process_file("\n10\na"), 10);
    assert_eq!(process_file("10\n10\n10"), 30);
    assert_eq!(process_file("a1b0c\na1b0c\na1b0c"), 30);
    assert_eq!(
        process_file("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"),
        142
    );
}
