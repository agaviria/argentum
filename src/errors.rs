//
// Errors
//
use std::io;
use std::fmt;
use std::error;
use std::error::Error;


#[derive(Debug)]
pub enum ParseError {
    // UnexpectedEOF returns when an operation could not be completed because an
    // "end of file" was reached prematurely.
    //
    // This typically means that an operation could only succeed if it read a
    // particular number of bytes but only a smaller number of bytes could be read.
    UnexpectedEOF
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedEOF => write!(f, "{}", self.description())
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnexpectedEOF => "Unexpected end of file",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}

impl From<ParseError> for ProcessError {
    fn from(err: ParseError) -> Self {
        ProcessError::ParseError(err)
    }
}

// Wrapper for many kinds of errors
#[derive(Debug)]
pub enum ProcessError {
    IOError(io::Error),
    ParseError(ParseError),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProcessError::IOError(ref err) => write!(f, "Input Error: {}", err),
            ProcessError::ParseError(ref err) => write!(f, "EOF: {}", err),
        }
    }
}

impl error::Error for ProcessError {
    fn description(&self) -> &str {
        match *self {
            ProcessError::IOError(ref err) => err.description(),
            ProcessError::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ProcessError::IOError(ref err) => Some(err),
            ProcessError::ParseError(ref err) => Some(err),
        }
    }
}

// Convert everything else into Error
//
impl From<io::Error> for ProcessError {
    fn from(err: io::Error) -> Self {
        ProcessError::IOError(err)
    }
}

// Convert Error into a general IO Error
//
impl From<ProcessError> for io::Error {
    fn from(err: ProcessError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
