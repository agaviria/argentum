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
    String(String),
    Number(f64),
    Comment(String),

    // Keywords.
    ElseKw,
    FalseKw,
    ForKw,
    FuncKw,
    IfKw,
    NilKw,
    PrintKw,
    ReturnKw,
    StructKw,
    SelfKw,
    TrueKw,
    VarKw,
    VoidKw,
    WhileKw,
}

impl fmt::Display for TokenRule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenRule::Ampersand          => write!(fmt, "&"),
            TokenRule::Asterik            => write!(fmt, "*"),
            TokenRule::Comma              => write!(fmt, ","),
            TokenRule::Colon              => write!(fmt, ":"),
            TokenRule::Dot                => write!(fmt, "."),
            TokenRule::FwdSlash           => write!(fmt, "/"),
            TokenRule::LeftParen          => write!(fmt, "("),
            TokenRule::RightParen         => write!(fmt, ")"),
            TokenRule::LeftBrace          => write!(fmt, "{{"),
            TokenRule::RightBrace         => write!(fmt, "}}"),
            TokenRule::LeftSquare         => write!(fmt, "["),
            TokenRule::RightSquare        => write!(fmt, "]"),
            TokenRule::Percentage         => write!(fmt, "%"),
            TokenRule::SemiColon          => write!(fmt, ";"),
            TokenRule::QuestionMark       => write!(fmt, "?"),
            TokenRule::Bang               => write!(fmt, "!"),
            TokenRule::BangEqual          => write!(fmt, "!="),
            TokenRule::Cast               => write!(fmt, "->"),
            TokenRule::Equal              => write!(fmt, "="),
            TokenRule::EqualEqual         => write!(fmt, "=="),
            TokenRule::GreaterThan        => write!(fmt, ">"),
            TokenRule::GreaterThanOrEq    => write!(fmt, ">="),
            TokenRule::LessThan           => write!(fmt, "<"),
            TokenRule::LessThanOrEq       => write!(fmt, "<="),
            TokenRule::LogicalAnd         => write!(fmt, "&&"),
            TokenRule::LogicalOr          => write!(fmt, "||"),
            TokenRule::Minus              => write!(fmt, "-"),
            TokenRule::MinusMinus         => write!(fmt, "--"),
            TokenRule::Path               => write!(fmt, "::"),
            TokenRule::Pipe               => write!(fmt, "|"),
            TokenRule::Plus               => write!(fmt, "+"),
            TokenRule::PlusPlus           => write!(fmt, "++"),

            TokenRule::Identifier(ref id) => id.fmt(fmt),
            TokenRule::String(ref s)      => write!(fmt, "\"{}\"", s),
            TokenRule::Number(ref num)    => num.fmt(fmt),
            TokenRule::Comment(ref c)     => write!(fmt, "{}", c),

            TokenRule::ElseKw             => write!(fmt, "else"),
            TokenRule::FalseKw            => write!(fmt, "false"),
            TokenRule::FuncKw             => write!(fmt, "fn"),
            TokenRule::ForKw              => write!(fmt, "for"),
            TokenRule::IfKw               => write!(fmt, "if"),
            TokenRule::NilKw              => write!(fmt, "nil"),
            TokenRule::PrintKw            => write!(fmt, "Print"),
            TokenRule::ReturnKw           => write!(fmt, "Return"),
            TokenRule::StructKw           => write!(fmt, "struct"),
            TokenRule::SelfKw             => write!(fmt, "self"),
            TokenRule::TrueKw             => write!(fmt, "true"),
            TokenRule::VarKw              => write!(fmt, "var"),
            TokenRule::VoidKw             => write!(fmt, "void"),
            TokenRule::WhileKw            => write!(fmt, "while"),
        }
    }
}
