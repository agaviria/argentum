use std::fmt;

// TODO: Would like to have Number be of type rug::float.
// Problem is that rug::float does not implement the Copy trait.  Look into it!

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'i> {
    // Single character tokens.
    Ampersand,
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
    Star,
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
    Identifier(&'i str),
    String(&'i str),
    Number(f64),

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

impl<'i> fmt::Display for Token<'i> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Ampersand          => write!(fmt, "&"),
            Token::Comma              => write!(fmt, ","),
            Token::Colon              => write!(fmt, ":"),
            Token::Dot                => write!(fmt, "."),
            Token::FwdSlash           => write!(fmt, "/"),
            Token::LeftParen          => write!(fmt, "("),
            Token::RightParen         => write!(fmt, ")"),
            Token::LeftBrace          => write!(fmt, "{{"),
            Token::RightBrace         => write!(fmt, "}}"),
            Token::LeftSquare         => write!(fmt, "["),
            Token::RightSquare        => write!(fmt, "]"),
            Token::Percentage         => write!(fmt, "%"),
            Token::SemiColon          => write!(fmt, ";"),
            Token::Star               => write!(fmt, "*"),
            Token::QuestionMark       => write!(fmt, "?"),
            Token::Bang               => write!(fmt, "!"),
            Token::BangEqual          => write!(fmt, "!="),
            Token::Cast               => write!(fmt, "->"),
            Token::Equal              => write!(fmt, "="),
            Token::EqualEqual         => write!(fmt, "=="),
            Token::GreaterThan        => write!(fmt, ">"),
            Token::GreaterThanOrEq    => write!(fmt, ">="),
            Token::LessThan           => write!(fmt, "<"),
            Token::LessThanOrEq       => write!(fmt, "<="),
            Token::LogicalAnd         => write!(fmt, "&&"),
            Token::LogicalOr          => write!(fmt, "||"),
            Token::Minus              => write!(fmt, "-"),
            Token::MinusMinus         => write!(fmt, "--"),
            Token::Path               => write!(fmt, "::"),
            Token::Pipe               => write!(fmt, "|"),
            Token::Plus               => write!(fmt, "+"),
            Token::PlusPlus           => write!(fmt, "++"),

            Token::Identifier(ref id) => id.fmt(fmt),
            Token::String(ref s)      => write!(fmt, "\"{}\"", s),
            Token::Number(ref num)    => num.fmt(fmt),

            Token::ElseKw             => write!(fmt, "else"),
            Token::FalseKw            => write!(fmt, "false"),
            Token::FuncKw             => write!(fmt, "fn"),
            Token::ForKw              => write!(fmt, "for"),
            Token::IfKw               => write!(fmt, "if"),
            Token::NilKw              => write!(fmt, "nil"),
            Token::PrintKw            => write!(fmt, "Print"),
            Token::ReturnKw           => write!(fmt, "Return"),
            Token::StructKw           => write!(fmt, "struct"),
            Token::SelfKw             => write!(fmt, "self"),
            Token::TrueKw             => write!(fmt, "true"),
            Token::VarKw              => write!(fmt, "var"),
            Token::VoidKw             => write!(fmt, "void"),
            Token::WhileKw            => write!(fmt, "while"),
        }
    }
}
