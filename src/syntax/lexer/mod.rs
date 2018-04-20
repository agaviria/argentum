pub mod token;

use std::str::CharIndices;
use self::token::Token;
use errors::{LexicalError, Position};

#[inline]
fn is_id_start(chr: char) -> bool {
  chr == '_' || chr.is_ascii_alphabetic()
}

#[inline]
fn is_id_continue(chr: char) -> bool {
  chr == '_' || chr.is_ascii_digit()
}

/// Lexer turns a string of source text into an iterator of tokens.  These
/// tokens will then be fed into the silver parser.
pub struct Lexer<'i> {
  source: &'i str,
  chars: CharIndices<'i>,
  peek: Option<(usize, char)>,
  peek_ahead: Option<(usize, char)>,
}

pub type SpanResult<'i> = Result<(Position, Token<'i>, Position), LexicalError>;

impl<'i> Lexer<'i> {
  pub fn new(source: &'i str) -> Lexer<'i> {
    // returns an iterator over the chars string slice and their positions.
    let mut chars = source.char_indices();
    let peek = chars.next();
    let peek_ahead = chars.next();

    Lexer {
      source,
      chars,
      peek,
      peek_ahead,
    }

  }

  /// bump peeks one character ahead of previous char string slice and returns it.
  /// string slice must be valid UTF-8 value, char represents unicode scalar value.
  fn bump(&mut self) -> Option<(usize, char)> {
    let next = self.peek;
    self.peek = self.peek_ahead;
    self.peek_ahead = self.chars.next();
    next
  }

  fn take_until<F>(&mut self, mut terminate: F) -> Option<usize> where F: FnMut(char) -> bool {
    while let Some((i, chr)) = self.peek {
      if terminate(chr) {
        return Some(i);
      } else {
        self.bump();
      }
    }
    None
  }

  /// Creates an iterator that yields elements based on a predicate.  Takes a
  /// closure as an argument.  It calls this closure on each element of the
  /// iterator, and yield elements while it returns true.
  fn take_while<F>(&mut self, mut condition: F) -> Option<usize> where F: FnMut(char) -> bool {
    self.take_until(|chr| !condition(chr))
  }

  /// skip_whitespace iterates over each element of the bool type `is_whitespace`
  /// as a closure. Returns false when it's job is over.
  fn skip_whitespace(&mut self) {
    self.take_while(|chr| chr.is_whitespace());
  }

  /// skip_to_line_end iterates over each element of the bool type `\n` line break
  /// as a closure. Returns false when it's job is over.
  fn skip_to_line_end(&mut self) {
    self.take_while(|chr| chr != '\n');
  }

  fn read_string(&mut self, position: usize) -> SpanResult<'i> {
    match self.take_until(|chr| chr == '"') {
      Some(i) => {
        self.bump();
        Ok((position, Token::String(&self.source[position + 1..i]), i + 1))
      }
      None => Err(LexicalError::UnterminatedString { pos: position }),
    }
  }

  fn read_number(&mut self, position: usize) -> SpanResult<'i> {
    let mut end = self.take_while(|chr| chr.is_ascii_digit());

    if let Some((_, '.')) = self.peek {
      // Check if it's a decimal or a field access.
      if let Some((_, next_chr)) = self.peek_ahead {
        if next_chr.is_ascii_digit() {
          self.bump();
          end = self.take_while(|chr| chr.is_ascii_digit());
        }
      }
    }

    let end = end.unwrap_or_else(|| self.source.len());

    Ok((
        position,
        Token::Number(self.source[position..end].parse().expect("unparsable number")),
        end,
        ))
  }

  fn read_identifier(&mut self, position: usize) -> SpanResult<'i> {
    let end = self.take_while(|chr| is_id_start(chr) || is_id_continue(chr))
      .unwrap_or_else(|| self.source.len());

    match &self.source[position..end] {
      "else"   => Ok((position, Token::ElseKw, end)),
      "false"  => Ok((position, Token::FalseKw, end)),
      "for"    => Ok((position, Token::ForKw, end)),
      "func"   => Ok((position, Token::FuncKw, end)),
      "if"     => Ok((position, Token::IfKw, end)),
      "nil"    => Ok((position, Token::NilKw, end)),
      "print"  => Ok((position, Token::PrintKw, end)),
      "return" => Ok((position, Token::ReturnKw, end)),
      "struct" => Ok((position, Token::StructKw, end)),
      "self"   => Ok((position, Token::SelfKw, end)),
      "true"   => Ok((position, Token::TrueKw, end)),
      "let"    => Ok((position, Token::VarKw, end)),
      "void"   => Ok((position, Token::VoidKw, end)),
      "while"  => Ok((position, Token::WhileKw, end)),
      id       => Ok((position, Token::Identifier(id), end))
    }
  }
}

impl<'i> Iterator for Lexer<'i> {
  type Item = SpanResult<'i>;

  fn next(&mut self) -> Option<SpanResult<'i>> {
    self.skip_whitespace();

    if let Some((i, chr)) = self.bump() {
      match chr {
        '(' => Some(Ok((i, Token::LeftParen, i + 1))),
        ')' => Some(Ok((i, Token::RightParen, i + 1))),
        '{' => Some(Ok((i, Token::LeftBrace, i + 1))),
        '}' => Some(Ok((i, Token::RightBrace, i + 1))),
        '[' => Some(Ok((i, Token::LeftSquare, i + 1))),
        ']' => Some(Ok((i, Token::RightSquare, i + 1))),
        ';' => Some(Ok((i, Token::SemiColon, i + 1))),
        '.' => Some(Ok((i, Token::Dot, i + 1))),
        '*' => Some(Ok((i, Token::Star, i + 1))),
        '%' => Some(Ok((i, Token::Percentage, i + 1))),
        '?' => Some(Ok((i, Token::QuestionMark, i + 1))),

        '/' => {
          // Iterate over a comment.
          if let Some((_, '/')) = self.peek {
            self.skip_to_line_end();
            self.next()
          } else {
            Some(Ok((i, Token::FwdSlash, i + 1)))
          }
        }

        ':' => {
          if let Some((_, ':')) = self.peek {
            self.bump();
            Some(Ok((i, Token::Path, i + 2)))
          } else {
            Some(Ok((i, Token::Colon, i + 1)))
          }
        }


        '!' => {
          if let Some((_, '=')) = self.peek {
            self.bump();
            Some(Ok((i, Token::BangEqual, i + 2)))
          } else {
            Some(Ok((i, Token::Bang, i + 1)))
          }
        }

        '=' => {
          if let Some((_, '=')) = self.peek {
            self.bump();
            Some(Ok((i, Token::EqualEqual, i + 2)))
          } else {
            Some(Ok((i, Token::Equal, i + 1)))
          }
        }

        // '-' => {
        //   while let Some(chr) = self.peek {
        //     match chr {
        //       '>'  => Some(Ok((i, self.transpose_char(Token::Cast), i + 2))),
        //       '-'  => Some(Ok((i, self.transpose_char(Token::MinusMinus), i + 2))),
        //       None => Some(Ok((i, self.transpose_char(Token::Minus), i + 1)))
        //     };
        //   }
        '-' => {
          if let Some((_, '>')) = self.peek {
            self.bump();
            Some(Ok((i, Token::Cast, i + 2)))
          } else {
            Some(Ok((i, Token::Minus, i + 1)))
          }
        }

        '+' => {
          if let Some((_, '+')) = self.peek {
            self.bump();
            Some(Ok((i, Token::PlusPlus, i + 2)))
          } else {
            Some(Ok((i, Token::Plus, i + 1)))
          }
        }

        '<' => {
          if let Some((_, '=')) = self.peek {
            self.bump();
            Some(Ok((i, Token::LessThanOrEq, i + 2)))
          } else {
            Some(Ok((i, Token::LessThan, i + 1)))
          }
        }

        '&' => {
          if let Some((_, '&')) = self.peek {
            self.bump();
            Some(Ok((i, Token::LogicalAnd, i + 2)))
          } else  {
            Some(Ok((i, Token::Ampersand, i + 1)))
          }
        }

        '|' => {
          if let Some((_, '|')) = self.peek {
            self.bump();
            Some(Ok((i, Token::LogicalOr, i + 2)))
          } else  {
            Some(Ok((i, Token::Pipe, i + 1)))
          }
        }

        '"' => Some(self.read_string(i)),
        chr if is_id_start(chr) => Some(self.read_identifier(i)),
        chr if chr.is_ascii_digit() => Some(self.read_number(i)),

        chr => Some(Err(LexicalError::InvalidCharacter { chr, pos: i }))
      }
      } else {
        None
      }
    }

  }
