use itertools::{multipeek, MultiPeek};
use std::str;

#[derive(Debug, Clone, Copy)]
/// Represents a position in the source file
/// Both line and column are represented by a 1-based index,
/// since this struct exists only to provide context to a user.
pub struct Position {
    // Represents a single line.
    ln: usize,
    // Represents a byte column.
    col: usize,
}

impl Position {
    // Initial starting position in source file.
    fn starting_point() -> Position {
        Position { ln: 1, col: 1 }
    }

    // Increment column byte iterator.
    fn increment_col(&mut self) -> () {
        self.col += 1;
    }

    // Increment line break iterator.
    fn increment_ln(&mut self) -> () {
        self.ln += 1;
        self.col = 1;
    }
}


// TODO:  * Figure out the input process.
//
// Possibly, convert the input token stream into an abstract syntax list representing
// the totality of the source code.  This result is then fed into a semantic
// analysis, returning a list of nodes for further processing, or an error.

#[allow(dead_code)]
pub struct Scanner<'a> {
    // The source code file.
    source_code: MultiPeek<str::Chars<'a>>,
    // current_lexeme is heap allocated, growable and not null terminated.
    current_lexeme: String,
    // input string to process
    input: &'a str,
    // Position of the first token character.
    current_position: Position,
}

impl<'a> Scanner<'a> {
    // Returns a new scanner over the provided input buffer.
    pub fn new(source: &'a str, input: &'a str) -> Self {
        Scanner {
            source_code: multipeek(source.chars()),
            current_lexeme: "".into(),
            input: input,
            current_position: Position::starting_point(),
        }
    }
}
