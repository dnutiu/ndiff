use crate::line::Line;
use std::fs::read_to_string;

mod line;

fn main() {
    let left_file = read_to_string("a.txt").expect("Left file not found");
    let right_file = read_to_string("b.txt").expect("Right file not found");

    let lines: Vec<Line> = line::compare_lines(&left_file, &right_file);

    let mut differences_counter: i32 = 0;
    for line in lines.iter() {
        line.print();
        match line {
            Line::MatchedLine(_, _) => {}
            Line::DifferingLine(_, _, _) => differences_counter += 1,
        }
    }
    println!();
    println!("Found {} differences.", differences_counter);
}
