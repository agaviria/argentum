use std::fmt;

use utils::{Span, Position};

// TODO: Would like to have Number be of type rug::float.
// Problem is that rug::float does not implement the Copy trait.  Look into it!

/// Represents a range of characters representing a lexical token within a source.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The kind of Token.
    pub kind: TokenRule,
    /// Span position (start, end)
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenRule, start: Position, end: Position) -> Token {
        Token {
            kind: kind,
            span: Span(start, end),
        }
    }
}

/// TokenRule is the token type variants the lexer stream state will match on.
#[derive(Debug, PartialEq)]
pub enum TokenRule {
    // Single character tokens.
    Ampersand,
    Asterik,
    Comma,
    Colon,
    Dot,
    FwdSlash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Percentage,
    SemiColon,
    QuestionMark,

    // One or two character tokens.
    Bang,
    BangEqual,
    Cast,
    Equal,
    EqualEqual,
    GreaterThan,
    GreaterThanOrEq,
    LessThan,
    LessThanOrEq,
    LogicalAnd,
    LogicalOr,
    Minus,
    MinusMinus,
    Path,
    Pipe,
    Plus,
    PlusPlus,

    // Literals.
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    // Comment(String),

    // Keywords.
    ElseKw,
    FalseKw,
    ForKw,
    FnKw,
    IfKw,
    NilKw,
    PrintKw,
    PubKw,
    ReturnKw,
    StructKw,
    SelfKw,
    SpecKw,
    TrueKw,
    LetKw,
    ModelKw,
    MountKw,
    WhileKw,
}

impl fmt::Display for TokenRule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenRule::Ampersand              => write!(fmt, "&"),
            TokenRule::Asterik                => write!(fmt, "*"),
            TokenRule::Comma                  => write!(fmt, ","),
            TokenRule::Colon                  => write!(fmt, ":"),
            TokenRule::Dot                    => write!(fmt, "."),
            TokenRule::FwdSlash               => write!(fmt, "/"),
            TokenRule::LeftParen              => write!(fmt, "("),
            TokenRule::RightParen             => write!(fmt, ")"),
            TokenRule::LeftBrace              => write!(fmt, "{{"),
            TokenRule::RightBrace             => write!(fmt, "}}"),
            TokenRule::LeftSquare             => write!(fmt, "["),
            TokenRule::RightSquare            => write!(fmt, "]"),
            TokenRule::Percentage             => write!(fmt, "%"),
            TokenRule::SemiColon              => write!(fmt, ";"),
            TokenRule::QuestionMark           => write!(fmt, "?"),
            TokenRule::Bang                   => write!(fmt, "!"),
            TokenRule::BangEqual              => write!(fmt, "!="),
            TokenRule::Cast                   => write!(fmt, "->"),
            TokenRule::Equal                  => write!(fmt, "="),
            TokenRule::EqualEqual             => write!(fmt, "=="),
            TokenRule::GreaterThan            => write!(fmt, ">"),
            TokenRule::GreaterThanOrEq        => write!(fmt, ">="),
            TokenRule::LessThan               => write!(fmt, "<"),
            TokenRule::LessThanOrEq           => write!(fmt, "<="),
            TokenRule::LogicalAnd             => write!(fmt, "&&"),
            TokenRule::LogicalOr              => write!(fmt, "||"),
            TokenRule::Minus                  => write!(fmt, "-"),
            TokenRule::MinusMinus             => write!(fmt, "--"),
            TokenRule::Path                   => write!(fmt, "::"),
            TokenRule::Pipe                   => write!(fmt, "|"),
            TokenRule::Plus                   => write!(fmt, "+"),
            TokenRule::PlusPlus               => write!(fmt, "++"),

            TokenRule::Identifier(ref id)     => id.fmt(fmt),
            TokenRule::StringLiteral(ref s)   => write!(fmt, "\"{}\"", s),
            TokenRule::NumberLiteral(ref num) => num.fmt(fmt),

            TokenRule::ElseKw                 => write!(fmt, "else"),
            TokenRule::FalseKw                => write!(fmt, "false"),
            TokenRule::FnKw                   => write!(fmt, "fn"),
            TokenRule::ForKw                  => write!(fmt, "for"),
            TokenRule::IfKw                   => write!(fmt, "if"),
            TokenRule::LetKw                  => write!(fmt, "let"),
            TokenRule::ModelKw                => write!(fmt, "model"),
            TokenRule::MountKw                => write!(fmt, "mount"),
            TokenRule::NilKw                  => write!(fmt, "nil"),
            TokenRule::PrintKw                => write!(fmt, "print"),
            TokenRule::PubKw                  => write!(fmt, "pub"),
            TokenRule::ReturnKw               => write!(fmt, "return"),
            TokenRule::SelfKw                 => write!(fmt, "self"),
            TokenRule::SpecKw                 => write!(fmt, "spec"),
            TokenRule::StructKw               => write!(fmt, "struct"),
            TokenRule::TrueKw                 => write!(fmt, "true"),
            TokenRule::WhileKw                => write!(fmt, "while"),
        }
    }
}

pub fn keyword_dict(string: &str) -> Option<TokenRule> {
    match string {
        "else"   => Some(TokenRule::ElseKw),
        "false"  => Some(TokenRule::FalseKw),
        "fn"     => Some(TokenRule::FnKw),
        "if"     => Some(TokenRule::IfKw),
        "let"    => Some(TokenRule::LetKw),
        "model"  => Some(TokenRule::ModelKw),  // trait equivalent.
        "mount"  => Some(TokenRule::MountKw),  // impl equivalent.
        "nil"    => Some(TokenRule::NilKw),
        "print"  => Some(TokenRule::PrintKw),
        "pub"    => Some(TokenRule::PubKw),
        "return" => Some(TokenRule::ReturnKw),
        "self"   => Some(TokenRule::SelfKw),
        "spec"   => Some(TokenRule::SpecKw),   // enum equivalent.
        "struct" => Some(TokenRule::StructKw),
        "true"   => Some(TokenRule::TrueKw),
        "while"  => Some(TokenRule::WhileKw),
        _        => None
    }
}
