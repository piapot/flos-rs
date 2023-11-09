mod token;

pub use token::{Token, TokenKind, TokenSpan};

#[derive(Debug)]
pub struct Tokenizer {
    input: Vec<u8>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: Vec::from(input),
            cursor: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(token) = self.next() {
            tokens.push(token);
        }

        tokens
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some(char) = self.input.get(self.cursor) {
            match *char {
                b'\n' => self.tokenize_linefeed(),
                c if c != b'\n' && c.is_ascii_whitespace() => {
                    let f = |c: u8| c.is_ascii_whitespace() && c != b'\n';
                    self.take_while(TokenKind::Whitespace, f)
                }
                c if c.is_ascii_digit() => {
                    let f = |c: u8| c.is_ascii_digit();
                    self.take_while(TokenKind::Digit, f)
                }
                c if c.is_ascii_alphabetic() => {
                    let f = |c: u8| c.is_ascii_alphabetic();
                    self.take_while(TokenKind::Word, f)
                }
                c if c.is_ascii_punctuation() => {
                    let f = |c: u8| c.is_ascii_punctuation();
                    self.take_while(TokenKind::Punctuation, f)
                }
                _ => {
                    let f = |c: u8| {
                        !(c.is_ascii_whitespace()
                            || c.is_ascii_digit()
                            || c.is_ascii_alphabetic()
                            || c.is_ascii_punctuation())
                    };
                    self.take_while(TokenKind::Unknown, f)
                }
            }
        } else {
            None
        }
    }

    fn tokenize_linefeed(&mut self) -> Option<Token> {
        self.input.get(self.cursor).map(|text| {
            let kind = TokenKind::Linefeed;
            let text = vec![*text];
            let line = self.line;
            let column = self.column;
            let start = self.cursor;
            let end = start + 1;
            self.cursor = end;
            self.line += 1;
            self.column = 1;
            let span = TokenSpan::new(line, column, start, end);
            Token::new(kind, text, span)
        })
    }

    fn take_while(&mut self, kind: TokenKind, f: fn(u8) -> bool) -> Option<Token> {
        let start = self.cursor;
        let mut chars = vec![];

        while let Some(char) = self.input.get(self.cursor) {
            if f(*char) {
                self.cursor += 1;
                chars.push(*char);
            } else {
                break;
            }
        }

        let length = chars.len();

        if length != 0 {
            let text = chars;
            let line = self.line;
            let column = self.column;
            let end = start + length;
            let span = TokenSpan::new(line, column, start, end);
            self.column += text.len();
            Some(Token::new(kind, text, span))
        } else {
            None
        }
    }
}
