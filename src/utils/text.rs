use super::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    // Represents a single line.
    ln: usize,
    // Represents a byte column.
    col: usize,
}

impl Position {
    pub fn new(ln: usize, col: usize) -> Position {
        Position { ln, col }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ContextSpan {
    pub position: Position,
    pub length: usize,
}

impl ContextSpan {
    pub fn new(position: Position, length: usize) -> ContextSpan {
        ContextSpan { position, length }
    }
}

pub struct FullTextSpan {
    pub item_path: Path,
    pub span: ContextSpan
}

impl FullTextSpan {
    pub fn new(item_path: Path, span: ContextSpan) -> FullTextSpan {
        FullTextSpan { item_path, span }
    }
}
