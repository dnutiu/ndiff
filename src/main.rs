use crate::line::{Line, MissingLineIndicator};
use itertools::Itertools;
use std::fs::read_to_string;

mod line;

/// Gets the lines of two given texts
///
/// It returns a [`Vec<Box<Line>>`]
fn get_lines(left_text: &String, right_text: &String) -> Vec<Line> {
    left_text
        .lines()
        .map(String::from)
        .zip_longest(right_text.lines().map(String::from))
        .enumerate()
        .map(|(line_number, item)| -> Line {
            let missing_line_indicator: MissingLineIndicator = Default::default();
            let left = item.clone().left().unwrap_or(missing_line_indicator.value.clone());
            let right = item.right().unwrap_or(missing_line_indicator.value);

            if left != right {
                Line::DifferingLine(line_number as i32 + 1, left, right)
            } else {
                Line::MatchedLine(line_number as i32 + 1, left)
            }
        })
        .collect()
}

fn main() {
    let left_file = read_to_string("a.txt").expect("Left file not found");
    let right_file = read_to_string("b.txt").expect("Right file not found");

    let lines: Vec<Line> = get_lines(&left_file, &right_file);

    let mut differences_counter: i32 = 0;
    for line in lines.iter() {
        line.print();
        match line {
            Line::MatchedLine(_, _) => {}
            Line::DifferingLine(_, _, _) => {
                differences_counter += 1
            }
        }
    }
    println!();
    println!("Found {} differences.", differences_counter);
}
