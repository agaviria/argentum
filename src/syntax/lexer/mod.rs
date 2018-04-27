//! The "lexer" module provides a token stream for argentum programs.
//! The main exports of this module are token definitions and the Lexer
//! struct, which is an iterator that yields 'Silver' tokens.
//!
//! ## Tokens
//! Tokens are the building blocks upon which the parser builds an
//! abstract syntax tree. A token represents a single syntactical
//! item within the language - a left parenthesis, an integer literal,
//! the `fn` keyword, and so on. The `Token` struct (and `TokenRule` enum)
//! together represent the tokens the lexer generates, in which the parser
//! eventually consumes.
//!
//! A token consists of a span and a variant. The span indicates the position
//! of the token in the source file - it is composed of both a start position
//! and a stop position, each of which are composed of a column and a line number.
//! The variant is a member of the TokenRule enum that indicates which type of
//! token this token is and, if applicable, contains some data as to what the
//! token represents.  This is used for literals and identifiers, which contain
//! the scanned value of the data type that each represents.
//!
//! ## The Lexer
//! The lexer is a glorified state machine, where each function roughly
//! corresponds to a state.  This state machine is exposed as the Lexer struct,
//! which exposes no public methods but instead implements the Iterator trait.
//! Every call to `next` runs the state machine on the input until it reaches
//! an accept state, reaches an EOF, or has some other sort of error.
//! `next` returns either a Token upon success, or a CompileDiagnostic on failure.
//! The parser is expected to abort parsing immediately if it encounters a lexer
//! error in this way.
//!
//! ## Lexical specification
//! There is no official lexical specification for 'Silver' yet, when the syntax
//! is solidified, details will be specified here.
pub mod token;

use std::iter::Peekable;
use std::string::String;
use std::str::Chars;

use self::token::Token;
use self::token::TokenRule::*;
use error;

use utils::{LexicalDiagnostic, Position, Severity, Span};

/// Lexer struct is used to perform the lexical scanning of a string.
pub struct Lexer<'a> {
	source:   String,              // source filename.
	iter:     Peekable<Chars<'a>>, // A buffer iterator.
	char_pos: Position             // current character position.
}

impl<'a> Iterator for Lexer<'a> {
	/// The type of the elements being iterated over.
	/// It returns a `Result`, where the `Ok` variant is a `Token`
	/// and the `Err`is a LexerError encountered while scanning the input.
	type Item = Result<Token, LexicalDiagnostic>;

	/// next `char` Lexer iterator.
	/// The state can branch into (3) Lexer states:
	///
	///	1) If the char is a newline `/n` increment char_position.0 += 1.
	///	2) If the current character is a comment char, skip line and continue.
	///	3) If the current character is not whitespace, we break out of the
	///	skip_comment_line_and_ws while loop and match on token_stream_state.

	///	Token stream state can be single character tokens, which are matched
	///	immediately or function methods defined on a peekable next char iterator.
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.iter.peek().clone() {
				Some('#') => self.skip_comment_line_and_ws(),
				Some(_)   => return Some(self.token_stream_state()),
				None      => return None
			}

		}
	}
}

impl<'a> Lexer<'a> {
	/// Creates a new lexer that is ready to lex an iterator stream of characters.
	/// takes in a source filename and a stream buffer of `chars`.
	pub fn new(source: String, stream: &'a str) -> Self {
		Lexer {
			source:   source,
			iter:     stream.chars().peekable(),
			char_pos: Position(1, 0)
		}
	}

	/// returns a reference of the source filename.
	pub fn filename(&self) -> &str {
		&self.source
	}

	/// bump advances the scanner. It increments the column position by one and
	/// returns the next `char`. If a newline is encountered, increment row
	/// char_pos. If iterator stream is empty it returns `None`.
	fn bump(&mut self) -> Option<(char)> {
		match self.iter.next() {
			Some('\n') => {
				self.char_pos.0 += 1;
				self.char_pos.1  = 0;
				Some('\n')
			},
			Some(chr) => {
				self.char_pos.1 += 1;
				Some(chr)
			},
			None => None
		}
	}

	/// Peeks at the next `char` from the iterator stream. It will advance the
	/// lexer if this function has not been called after a `bump`.
	fn peek_char(&mut self) -> Option<&char> {
		self.iter.peek()
	}

	/// Peeks at the next `char` and compares it to `prev`.
	/// Returns `true` if they are equal.
	fn peek_char_eq(&mut self, prev: char) -> bool {
		match self.peek_char() {
			Some(&chr) => chr == prev,
			None       => false,
		}
	}

	/// Reads the next `char` from iterator stream and discards it. It advances
	/// the scan to the next iter `char` read.
	fn skip(&mut self) {
		self.bump();
	}

	/// Skips all `chars` until it finds a newline (`\n`) or until the EOF.
	fn skip_line(&mut self) {
		while !self.peek_char_eq('\n') {
			self.skip();
		}
	}

	/// Increments `char` row position if `char` is a newline.  It skips line if
	/// iterator match is a comment defined char. If the `char` is
	/// not whitespace, the control flow loop ends and a bump() method is called
	/// to advance next character read.
	fn skip_comment_line_and_ws(&mut self) {
		while let Some(&chr) = self.iter.peek() {
			// if a newline is encountered, increment line position by one (1).
			if chr == '\n' {
				self.char_pos.0 += 1;
				self.char_pos.1  = 0;
			} else if chr == '/' && self.peek_char_eq('/') {
				// skip line if `char` is a comment.  Argentum uses '#' as the
				// comment `char` identifier token.
				self.skip_line();
				continue;
			}
			// if `char` is not whitespace, break out of for loop.
			if !chr.is_whitespace() {
				break;
			}
			// advances scan to next iter `char` read.
			self.bump();
		}
	}

	/// The initial token stream consumption state.
	fn token_stream_state(&mut self) -> Result<Token, LexicalDiagnostic> {
		let pos = self.char_pos;
		// single character tokens are scanned and returned immediately.
		match self.iter.next().unwrap() {
			'(' => Ok(Token::new(LeftParen, self.char_pos, self.char_pos)),
			')' => Ok(Token::new(RightParen, self.char_pos, self.char_pos)),
			'{' => Ok(Token::new(LeftBrace, self.char_pos, self.char_pos)),
			'}' => Ok(Token::new(RightBrace, self.char_pos, self.char_pos)),
			'[' => Ok(Token::new(LeftSquare, self.char_pos, self.char_pos)),
			']' => Ok(Token::new(RightSquare, self.char_pos, self.char_pos)),
			';' => Ok(Token::new(SemiColon, self.char_pos, self.char_pos)),
			'.' => Ok(Token::new(Dot, self.char_pos, self.char_pos)),
			'*' => Ok(Token::new(Asterik, self.char_pos, self.char_pos)),
			'%' => Ok(Token::new(Percentage, self.char_pos, self.char_pos)),
			'?' => Ok(Token::new(QuestionMark, self.char_pos, self.char_pos)),
			'-' => self.minus_or_cast_op(pos),
			_   => self.span_err(error::LexerErrorKind::UnknownCharacter, pos, self.char_pos)
		}
	}

	/// Emits a match on a minus operator '-' or cast operator '->'.
	fn minus_or_cast_op(&mut self, start: Position) -> Result<Token, LexicalDiagnostic> {
		if let Some(&'>') = self.iter.peek() {
			// It is a `->` token.
			let _ = self.bump();
			Ok(Token::new(Cast, start, self.char_pos))
		} else {
			// It is a `-` token.
			Ok(Token::new(Minus, start, self.char_pos))
		}
	}

	/// span_error returns a LexerError struct type.
	fn span_err<T>(&self, kind: error::LexerErrorKind, start: Position, end: Position)
		-> Result<T, LexicalDiagnostic> {
			Err(From::from(error::LexerError {
				source: self.source.clone(),
				span: Span(start, end),
				severity: Severity::Error,
				kind: kind
			}))
		}

}
