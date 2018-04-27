use std::fmt;

/// The position of a single character in a source document.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position (pub u32, pub u32);

/// Span provides access to the start and end position of last token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span(pub Position, pub Position);

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Span(Position(start_row, start_col), Position(end_row, end_col)) = *self;
        write!(f, "(row:{}, col:{}, row:{}, col:{})",
        start_row, start_col, end_row, end_col)
    }
}

/// Severity relates to the flag type error:
///   Waning |
///   Error  |
///   Fatal
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Severity {
    Warning,
    Error,
    Fatal
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
            Severity::Fatal => write!(f, "fatal")
        }
    }
}

/// LexicalDiagnostic provides display format used in Result trait.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LexicalDiagnostic {
    pub source: String,
    pub span: Span,
    pub severity: Severity,
    pub msg: String
}

impl fmt::Display for LexicalDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {}: {}",
               self.source,
               self.span,
               self.severity,
               self.msg)
    }
}
