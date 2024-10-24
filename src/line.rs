use std::{fmt};
use std::fmt::{Debug, Formatter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug)]
pub(crate) struct MissingLineIndicator {
    pub value: String
}

impl Default for MissingLineIndicator {
    fn default() -> Self {
        MissingLineIndicator{
            value: String::from("<missing line>")
        }
    }
}

/// The line enum models a file line.
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
                println!("{line_number}. ----");
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