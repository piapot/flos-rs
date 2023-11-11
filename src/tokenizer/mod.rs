mod token;

pub use token::{Token, TokenKind};

#[derive(Debug)]
pub struct Tokenizer {
    source: Vec<u8>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source,
            cursor: 0,
            line: 1,   // >= 1
            column: 0, // >= 0
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
        if let Some(char) = self.source.get(self.cursor) {
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
        self.source.get(self.cursor).map(|text| {
            let kind = TokenKind::Linefeed;
            let text = vec![*text];
            let line = self.line;
            let start_column = self.column;
            let end_column = self.column + 1;
            self.cursor += 1;
            self.line += 1;
            self.column = 0;
            Token::new(kind, text, line, [start_column, end_column])
        })
    }

    fn take_while(&mut self, kind: TokenKind, f: fn(u8) -> bool) -> Option<Token> {
        let mut chars = vec![];

        while let Some(char) = self.source.get(self.cursor) {
            if f(*char) {
                self.cursor += 1;
                chars.push(*char);
            } else {
                break;
            }
        }

        let chars_length = chars.len();

        if chars_length != 0 {
            let text = chars;
            let line = self.line;
            let start_column = self.column;
            let end_column = start_column + chars_length;
            self.column = end_column;
            Some(Token::new(kind, text, line, [start_column, end_column]))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let source = r#"
            x = 1
            y = 2
            z = x + y
        "#
        .trim();
        let mut tokenizer = Tokenizer::new(source.into());
        let tokens = tokenizer.scan();
        println!("{tokens:?}");
    }
}
