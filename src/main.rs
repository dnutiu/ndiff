use crate::line::{DifferingLine, Line, MatchedLine};
use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::Deref;

mod line;

/// Gets the lines of two given texts
///
/// It returns a [`Vec<Box<dyn Line>>`]
fn get_lines(left_text: &String, right_text: &String) -> Vec<Box<dyn Line>> {
    left_text
        .lines()
        .map(String::from)
        .zip_longest(right_text.lines().map(String::from))
        .enumerate()
        .map(|(line, item)| -> Box<dyn Line> {
            let left = item.clone().left().unwrap_or(String::from(""));
            let right = item.right().unwrap_or(String::from(""));

            if left != right {
                Box::new(DifferingLine::new(line as i32 + 1, left, right))
            } else {
                Box::new(MatchedLine::new(line as i32 + 1, left))
            }
        })
        .collect()
}

fn main() {
    let left_file = read_to_string("a.txt").expect("Left file not found");
    let right_file = read_to_string("b.txt").expect("Right file not found");

    let lines: Vec<Box<dyn Line>> = get_lines(&left_file, &right_file);

    let mut differences_counter: i32 = 0;
    for line in lines.iter() {
        let line = line.deref();
        print!("{}", line);
        if line.is_differing() {
            differences_counter += 1;
        }
    }
    println!();
    println!("Found {} differences.", differences_counter);
}
