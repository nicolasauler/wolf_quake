use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    IoError(std::io::ErrorKind),
}

impl From<std::num::ParseIntError> for ParsingError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<std::io::Error> for ParsingError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.kind())
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn a_not_found_error()(s in "\\PC*") -> ParsingError {
            ParsingError::NotFound(s)
        }
    }

    // fn a_random_parsing_error() -> BoxedStrategy<ParsingError> {
    //     prop_oneof![
    //         a_not_found_error(),
    //         any::<std::num::ParseIntError>().prop_map(ParsingError::ParseIntError),
    //         any::<std::io::ErrorKind>().prop_map(ParsingError::IoError),
    //     ]
    //     .boxed()
    // }

    fn a_parseint_parsing_error() -> BoxedStrategy<ParsingError> {
        any::<std::num::ParseIntError>()
            .prop_map(ParsingError::ParseIntError)
            .boxed()
    }

    fn an_io_error_parsing_error() -> BoxedStrategy<ParsingError> {
        any::<std::io::ErrorKind>()
            .prop_map(ParsingError::IoError)
            .boxed()
    }

    proptest! {
        #[test]
        fn test_parsing_error_from_parseint_error(parsing_error in a_parseint_parsing_error()) {
            let err: std::num::ParseIntError = match parsing_error.clone() {
                ParsingError::ParseIntError(err) => err,
                _ => panic!("Expected ParseIntError"),
            };
            assert_eq!(ParsingError::from(err), parsing_error);
        }
    }

    proptest! {
        #[test]
        fn test_parsing_error_from_io_error(parsing_error in an_io_error_parsing_error()) {
            let err: std::io::ErrorKind = match parsing_error.clone() {
                ParsingError::IoError(err) => err,
                _ => panic!("Expected IoError"),
            };
            assert_eq!(ParsingError::from(std::io::Error::from(err)), parsing_error);
        }
    }

    proptest! {
        #[test]
        fn test_display_not_found_error(parsing_error in a_not_found_error()) {
            let s = match parsing_error.clone() {
                ParsingError::NotFound(s) => s,
                _ => panic!("Expected NotFound"),
            };
            assert_eq!(format!("{}", parsing_error), format!("Not found: {s}"));
        }
    }

    proptest! {
        #[test]
        fn test_display_parseint_error(parsing_error in a_parseint_parsing_error()) {
            let err: std::num::ParseIntError = match parsing_error.clone() {
                ParsingError::ParseIntError(err) => err,
                _ => panic!("Expected ParseIntError"),
            };
            assert_eq!(format!("{}", parsing_error), format!("ParseIntError: {}", err));
        }
    }

    proptest! {
        #[test]
        fn test_display_io_error(parsing_error in an_io_error_parsing_error()) {
            let err: std::io::ErrorKind = match parsing_error.clone() {
                ParsingError::IoError(err) => err,
                _ => panic!("Expected IoError"),
            };
            assert_eq!(format!("{}", parsing_error), format!("IoError: {}", err));
        }
    }
}
