use itertools::Itertools;
use std::fmt;
use std::fmt::{Debug, Formatter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug)]
pub(crate) struct MissingLineIndicator {
    pub value: String,
}

impl Default for MissingLineIndicator {
    fn default() -> Self {
        MissingLineIndicator {
            value: String::from("<missing line>"),
        }
    }
}

/// The line enum models a file line.
#[derive(PartialEq, Debug)]
pub(crate) enum Line {
    /// MatchedLine represents a line that matches when comparing the files.
    ///
    /// MatchedLine(line_number, line)
    MatchedLine(i32, String),
    /// DifferingLine represents a line that does not match its counterpart line.
    ///
    /// DifferingLine(line_number, left_line, right_line)
    DifferingLine(i32, String, String),
}

impl Line {
    /// Prints the line to stdout
    pub fn print(&self) {
        match self {
            Line::MatchedLine(line_number, line) => {
                println!(r#"{}. {}"#, line_number, line)
            }
            Line::DifferingLine(line_number, left_line, right_line) => {
                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                println!(" - {}", left_line);
                let _ = stdout.reset();
                let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                print!("{}.", line_number);
                let _ = stdout.reset();
                println!("----");
                let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                println!(" + {}", right_line);
                let _ = stdout.reset();
                let _ = stdout.reset();
            }
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Line::MatchedLine(line_number, line) => {
                write!(f, r#"{}. {}"#, line_number, line,).expect("Failed to write MatchedLine");
                writeln!(f)
            }
            Line::DifferingLine(line_number, left_line, right_line) => {
                write!(
                    f,
                    r#" - {left_line}
{line_number}. ----
 + {right_line}"#,
                    left_line = left_line,
                    line_number = line_number,
                    right_line = right_line
                )
                .expect("Failed to write DifferingLines");
                writeln!(f)
            }
        }
    }
}

/// Compares the lines of two texts.
///
/// It returns a [`Vec<Line>`]
pub fn compare_lines(left_text: &str, right_text: &str) -> Vec<Line> {
    left_text
        .lines()
        .map(String::from)
        .zip_longest(right_text.lines().map(String::from))
        .enumerate()
        .map(|(line_number, item)| -> Line {
            let missing_line_indicator: MissingLineIndicator = Default::default();
            let left = item
                .clone()
                .left()
                .unwrap_or(missing_line_indicator.value.clone());
            let right = item.right().unwrap_or(missing_line_indicator.value);

            if left != right {
                Line::DifferingLine(line_number as i32 + 1, left, right)
            } else {
                Line::MatchedLine(line_number as i32 + 1, left)
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::line::Line::{DifferingLine, MatchedLine};

    #[test]
    fn it_should_compare_one_matched_line() {
        let lines = compare_lines("asd", "asd");
        assert_eq!(lines, vec![MatchedLine(1, String::from("asd"))])
    }

    #[test]
    fn it_should_compare_one_differing_line() {
        let lines = compare_lines("asd", "basd");
        assert_eq!(
            lines,
            vec![DifferingLine(1, String::from("asd"), String::from("basd"))]
        )
    }

    #[test]
    fn it_should_compare_one_matching_and_one_differing_line() {
        let lines = compare_lines("one\ntwo", "zero\ntwo");
        assert_eq!(
            lines,
            vec![
                DifferingLine(1, String::from("one"), String::from("zero")),
                MatchedLine(2, String::from("two"))
            ]
        )
    }
}
