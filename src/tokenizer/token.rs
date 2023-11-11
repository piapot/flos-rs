#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,
    Whitespace,
    Linefeed,
    Digit,
    Word,
    Punctuation,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    text: Vec<u8>,
    line: usize,
    column: [usize; 2],
}

impl Token {
    pub fn new(kind: TokenKind, text: Vec<u8>, line: usize, column: [usize; 2]) -> Self {
        Self {
            kind,
            text,
            line,
            column,
        }
    }
}
