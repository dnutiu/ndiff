use crate::difference::Difference;
use itertools::Itertools;
use std::fs::read_to_string;

mod difference;

/// Gets the differences of lines that differ in the given text.
///
/// It returns a [`Vec<Difference>`]
fn get_differences(left_text: &String, right_text: &String) -> Vec<Difference> {
    left_text
        .lines()
        .map(String::from)
        .zip_longest(right_text.lines().map(String::from))
        .enumerate()
        .filter_map(|(line, item)| -> Option<Difference> {
            let left = item.clone().left().unwrap_or(String::from(""));
            let right = item.right().unwrap_or(String::from(""));

            if left != right {
                Some(Difference::new(line as i32 + 1, left, right))
            } else {
                return None;
            }
        })
        .collect()
}

fn main() {
    let left_file =
        read_to_string("/Users/dnutiu/RustroverProjects/ndiff/a.txt").expect("Left file not found");
    let right_file =
        read_to_string("/Users/dnutiu/RustroverProjects/ndiff/b.txt").expect("Right file found");

    let res: Vec<Difference> = get_differences(&left_file, &right_file);

    for diff in res.iter() {
        print!("{diff}")
    }
    println!("Found {} differences.", res.len())
}
