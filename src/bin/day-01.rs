use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let process_string_numbers = &args[2] == "1";

    let calibration_document = fs::read_to_string(file_path).expect("The file should exist.");
    let result = process_file(&calibration_document, process_string_numbers);
    println!("Result: {}", result)
}

/// Process the content of the file.
fn process_file(file: &str, process_string_numbers: bool) -> i32 {
    let mut line_numbers: Vec<i32> = vec![];

    for line in file.split('\n') {
        println!("Line: {}", line);
        line_numbers.push(get_number_from_line(line, process_string_numbers))
    }

    let mut result = 0;
    for line_number in line_numbers {
        result += line_number
    }

    result
}

/// Get the two digit number from a line.
/// The number is found by combining the first and last number digit in the line.
fn get_number_from_line(line: &str, process_string_numbers: bool) -> i32 {
    let mut numbers: Vec<char> = vec![];
    let number_definitions: Vec<(String, char)> = vec![
        ("1".to_string(), '1'),
        ("one".to_string(), '1'),
        ("2".to_string(), '2'),
        ("two".to_string(), '2'),
        ("3".to_string(), '3'),
        ("three".to_string(), '3'),
        ("4".to_string(), '4'),
        ("four".to_string(), '4'),
        ("5".to_string(), '5'),
        ("five".to_string(), '5'),
        ("6".to_string(), '6'),
        ("six".to_string(), '6'),
        ("7".to_string(), '7'),
        ("seven".to_string(), '7'),
        ("8".to_string(), '8'),
        ("eight".to_string(), '8'),
        ("9".to_string(), '9'),
        ("nine".to_string(), '9'),
    ];

    if process_string_numbers {
        for i in 0..line.len() {
            let tmp = line.get(i..line.len()).unwrap_or("");

            for y in &number_definitions {
                if tmp.starts_with(&y.0) {
                    numbers.push(y.1)
                }
            }
        }
    } else {
        // Iterate over every character of the line and check if its numeric.
        // If it is, we push it to the numbers array.
        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char);
                println!("Found numeric charcter: {}", char)
            }
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
    assert_eq!(process_file("", false), 0);
    assert_eq!(process_file("a", false), 0);
    assert_eq!(process_file("1", false), 11);
    assert_eq!(process_file("12", false), 12);
    assert_eq!(process_file("\n10", false), 10);
    assert_eq!(process_file("a\n10", false), 10);
    assert_eq!(process_file("\n10\na", false), 10);
    assert_eq!(process_file("10\n10\n10", false), 30);
    assert_eq!(process_file("a1b0c\na1b0c\na1b0c", false), 30);
    assert_eq!(
        process_file("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", false),
        142
    );
}

#[test]
fn test_process_file_with_spelled_out_numbers() {
    assert_eq!(process_file("", true), 0);
    assert_eq!(process_file("a", true), 0);
    assert_eq!(process_file("1", true), 11);
    assert_eq!(process_file("one", true), 11);
    assert_eq!(process_file("1two", true), 12);
    assert_eq!(process_file("two1", true), 21);
    assert_eq!(process_file("two91", true), 21);
    assert_eq!(process_file("2two1", true), 21);
    assert_eq!(process_file("2two4four4three1", true), 21);
    assert_eq!(process_file("2two4four4three1one", true), 21);
    assert_eq!(
        process_file("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen", true),
        281
    );
}
