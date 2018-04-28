//
// Error includes LexerError and LexerErrorKind.
// It implements the From trait for LexicalDiagnostic struct which displays
// source filename, span position, severity of error and message struct fields.
//
use utils::{LexicalDiagnostic, Severity, Span};

/// LexerError includes all field items required by the LexicalDiagnostic struct.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LexerError {
    pub source:   String,
    pub span:     Span,
    pub severity: Severity,
    pub kind:     LexerErrorKind
}

/// LexerErrorKind holds all the error variants for the Lexer state.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LexerErrorKind {
    UnknownChar,
    InvalidNumericLiteral,
    UnterminatedStringLiteral,
    InvalidEscapeChar
}

impl From<LexerError> for LexicalDiagnostic {
    fn from(err: LexerError) -> LexicalDiagnostic {
        let message = match err.kind {
            LexerErrorKind::UnknownChar               => "unknown character",
            LexerErrorKind::InvalidNumericLiteral     => "invalid numeric literal",
            LexerErrorKind::UnterminatedStringLiteral => "unexpected EOF while scanning string literal",
            LexerErrorKind::InvalidEscapeChar         => "invalid escape character"
        };
        LexicalDiagnostic {
            source:   err.source,
            span:     err.span,
            severity: err.severity,
            msg:      message.to_string()
        }
    }
}
