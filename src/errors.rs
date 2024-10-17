use std::fmt;

/// Various errors that may rise when parsing RTTM plain-text files.
#[derive(Debug)]
pub enum RttmError {
    /// Unexpected number of segments in row. Must be 10.
    SegmentAlignmentError(usize),
    /// IO Error.
    IOError(std::io::Error),
    /// Parse string to float error.
    ParseFloatError(std::num::ParseFloatError),
    /// Parse string to integer error.
    ParseIntError(std::num::ParseIntError),
}

impl std::error::Error for RttmError {}
impl fmt::Display for RttmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SegmentAlignmentError(index) => write!(f, "Index overflow: expected 10 values, got {index}"),
            Self::IOError(err) => write!(f, "IO error: {err}"),
            Self::ParseFloatError(err) => write!(f, "Float parse error: {err}"),
            Self::ParseIntError(err) => write!(f, "Integer parse error: {err}"),
        }
    }
}

/// Converts std::io::Error to RttmError
impl From<std::io::Error> for RttmError {
    fn from(err: std::io::Error) -> RttmError {
        RttmError::IOError(err)
    }
}

/// Converts RttmError to std::io::Error
impl From<RttmError> for std::io::Error {
    fn from(err: RttmError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err)
    }
}

/// Converts std::num::ParseFloatError to RttmError
impl From<std::num::ParseFloatError> for RttmError {
    fn from(err: std::num::ParseFloatError) -> RttmError {
        RttmError::ParseFloatError(err)
    }
}

/// Converts std::num::ParseFloatError to RttmError
impl From<std::num::ParseIntError> for RttmError {
    fn from(err: std::num::ParseIntError) -> RttmError {
        RttmError::ParseIntError(err)
    }
}