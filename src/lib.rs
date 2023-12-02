use std::fs;

pub fn get_lines_from_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).expect("The file should exist.");
    let mut lines: Vec<String> = vec![];
    for line in file.split('\n') {
        println!("Line: {}", line);
        lines.push(line.to_string())
    }
    lines
}
