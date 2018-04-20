//
// Errors
//
pub type Position = usize;

/// LexicalError represents invalid lexical analysis error types.
/// The custom derive for Fail derives an impl of both Fail and Display.
#[derive(Debug, Fail, PartialEq)]
pub enum LexicalError {
    /// Unknown character was found in the source, such as `?` or `#`
    #[fail(display = "Invalid character '{}' found at {}", chr, pos)]
    InvalidCharacter { chr: char, pos: Position },

    /// Unknown character was found in the source, such as `?` or `#`
    #[fail(display = "Invalid escape character '{}' found at {}", chr, pos)]
    InvalidEscCharacter { chr: char, pos: Position },

    /// A string was not terminated.
    #[fail(display = "String starting at {} was not terminated", pos)]
    UnterminatedString { pos: Position },

    /// The source ended unexpectedly.
    #[fail(display = "Unexpected end of file encountered.")]
    UnexpectedEOF { pos: Position },
}
