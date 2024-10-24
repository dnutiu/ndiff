use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

pub trait Line: fmt::Display {
    fn is_differing(&self) -> bool;
}

/// Line represents a non-differing line.
pub(crate) struct MatchedLine {
    line_number: i32,
    line: String,
}

impl MatchedLine {
    /// Constructs a new Line instance
    pub fn new(line_number: i32, line: String) -> MatchedLine {
        MatchedLine { line_number, line }
    }

    /// get_line returns the line.
    pub fn get_line(&self) -> &String {
        &self.line
    }
}

impl fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"{}. {}"#, self.line_number, self.get_line(),)
            .expect("Failed to write MatchedLine");
        writeln!(f)
    }
}

impl Line for MatchedLine {
    fn is_differing(&self) -> bool {
       false
    }
}

/// A line difference is represented by the Difference struct.
#[derive(Debug)]
pub(crate) struct DifferingLine {
    line_number: i32,
    missing_line_indicator: String,
    left_line: String,
    right_line: String,
}

impl DifferingLine {
    /// Constructs a new difference instance.
    pub fn new(line_number: i32, left_line: String, right_line: String) -> DifferingLine {
        DifferingLine {
            line_number,
            missing_line_indicator: String::from("<missing line>"),
            left_line: left_line.into(),
            right_line: right_line.into(),
        }
    }

    /// Returns the left line of the difference.
    #[allow(dead_code)]
    pub fn left_line(&self) -> &String {
        if self.left_line.deref() == "" {
            return &self.missing_line_indicator;
        }
        &self.left_line
    }

    /// Returns the right line of the difference.
    #[allow(dead_code)]
    pub fn right_line(&self) -> &String {
        if self.right_line.deref() == "" {
            return &self.missing_line_indicator;
        }
        &self.right_line
    }
}

impl fmt::Display for DifferingLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"  - {}
{}. ----
  + {}"#,
            self.left_line(),
            self.line_number,
            self.right_line()
        )
        .expect("Failed to write DifferingLines");
        writeln!(f)
    }
}

impl Line for DifferingLine {
    fn is_differing(&self) -> bool {
        true
    }
}
