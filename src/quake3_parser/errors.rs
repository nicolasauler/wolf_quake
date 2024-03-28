use std::fmt::Display;

pub enum ParsingError {
    /// When an expected value from the log is not found
    /// (e.g. the `mean_id` in the Kill event)
    NotFound(String),
    /// When the parsing of a u32 fails
    /// (e.g. when parsing the `killer_id` in the Kill event)
    /// (`std::num::ParseIntError`)
    ParseIntError(std::num::ParseIntError),
    /// When an IO error occurs
    /// (e.g. when reading the file, if the filepath is invalid)
    IoError(std::io::Error),
}

impl From<std::num::ParseIntError> for ParsingError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<std::io::Error> for ParsingError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "Not found: {s}"),
            Self::ParseIntError(err) => write!(f, "ParseIntError: {err}"),
            Self::IoError(err) => write!(f, "IoError: {err}"),
        }
    }
}
