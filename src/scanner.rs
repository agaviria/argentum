use std::fs::File;
use std::path::PathBuf;
use std::str;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result as IOResult;

use itertools::{multipeek, MultiPeek};
use utils::Position;

// TODO:  * Figure out the input process.
//
// Possibly, convert the input token stream into an abstract syntax list representing
// the totality of the source code.  This result is then fed into a semantic
// analysis, returning a list of nodes for further processing, or an error.

#[allow(dead_code)]
pub struct Scanner<'a> {
    // file path buffer --optional
    pub path_buffer: Option<PathBuf>,
    // peekable iterator.
    pub peekable: MultiPeek<str::Chars<'a>>,
    pub next: Option<char>,
    pub next_position: Position,
}

impl<'a> Scanner<'a> {
    // Returns a new scanner over the provided input buffer.
    pub fn init_from_file(path_buffer: PathBuf, mut body: &'a mut String) -> IOResult<Scanner<'a>> {
        let file = File::open(&path_buffer)?;

        let mut buffer_reader = BufReader::new(file);
        buffer_reader.read_to_string(&mut body)?;

        let mut source_reader = Scanner::init_from_str(body);
        source_reader.path_buffer = Some(path_buffer);

        Ok(source_reader)
    }

    pub fn init_from_str(source: &'a str) -> Scanner<'a> {
        let mut peekable = multipeek(source.chars());
        let next = peekable.next();

        Scanner {
            path_buffer: None,
            peekable,
            next,
            next_position: Position::new(0, 0)
        }
    }
}
