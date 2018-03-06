mod token_kind;

use std::borrow::Cow;
use utils::{Position, ContextSpan};
use self::token_kind::TokenKind;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
    pub detail: String,
    pub lexeme: Cow<'static, str>
}

impl Token {
    pub fn new(kind: TokenKind, position: Position, detail: String, lexeme: Cow<'static, str>) -> Token {
        Token { kind, position, detail, lexeme }
    }

    pub fn span(&self) -> ContextSpan {
        ContextSpan {
            position: self.position,
            length: self.lexeme.len()
        }
    }
}
