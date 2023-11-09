#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Linefeed,
    Digit,
    Word,
    Punctuation,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    line: usize,
    column: usize,
    start: usize,
    end: usize,
}

impl TokenSpan {
    pub fn new(line: usize, column: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            column,
            start,
            end,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    text: Vec<u8>,
    span: TokenSpan,
}

impl Token {
    pub fn new(kind: TokenKind, text: Vec<u8>, span: TokenSpan) -> Self {
        Self { kind, text, span }
    }
}
