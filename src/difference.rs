use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

/// A line difference is represented by the Difference struct.
#[derive(Debug)]
pub(crate) struct Difference {
    line_number: i32,
    missing_line_indicator: String,
    left_line: String,
    right_line: String,
}

impl Difference {
    pub fn new(line_number: i32, left_line: String, right_line: String) -> Difference {
        Difference {
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

/// Implements the display trait.
impl fmt::Display for Difference {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Line: {}.
< {}
----
> {}

"#,
            self.line_number,
            self.left_line(),
            self.right_line()
        )
    }
}
