use crate::line::Line;
use clap::Parser;
use std::fs::read_to_string;
use std::process::exit;

mod line;

#[derive(Parser, Debug)]
#[command(
    version = "0.1",
    about = "A simple toy program to compare file differences.",
    long_about = "A simple toy program to compare file differences. Written as an exercise to learn Rust and its ecosystem."
)]
struct CliArgs {
    /// The path to the left file to be compared.
    left_file_path: String,

    /// The path to the right file to be compared.
    right_file_path: String,
}

fn main() {
    let args = CliArgs::parse();

    let left_file = match read_to_string(&args.left_file_path) {
        Ok(right_file) => right_file,
        Err(_) => {
            eprintln!("ERROR: File {} was not found.", &args.left_file_path);
            exit(1)
        }
    };

    let right_file = match read_to_string(&args.right_file_path) {
        Ok(right_file) => right_file,
        Err(_) => {
            eprintln!("ERROR: File {} was not found.", &args.right_file_path);
            exit(1)
        }
    };

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
