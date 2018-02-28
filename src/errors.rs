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

impl From<ParseError> for InputError {
    fn from(err: ParseError) -> Self {
        InputError::ParseError(err)
    }
}

// Wrapper for many kinds of errors
#[derive(Debug)]
pub enum InputError {
    IOError(io::Error),
    ParseError(ParseError),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::IOError(ref err) => write!(f, "Input Error: {}", err),
            InputError::ParseError(ref err) => write!(f, "EOF: {}", err),
        }
    }
}

impl error::Error for InputError {
    fn description(&self) -> &str {
        match *self {
            InputError::IOError(ref err) => err.description(),
            InputError::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            InputError::IOError(ref err) => Some(err),
            InputError::ParseError(ref err) => Some(err),
        }
    }
}

// Convert everything else into Error
//
impl From<io::Error> for InputError {
    fn from(err: io::Error) -> Self {
        InputError::IOError(err)
    }
}

// Convert Error into a general IO Error
//
impl From<InputError> for io::Error {
    fn from(err: InputError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
