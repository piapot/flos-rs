use crate::token::Token;

static SYNTAX_ERROR_MSG: &str = "Find Uncaught SyntaxError: Invalid or unexpected token";
static INDEX_ERROR_MSG: &str = "Find Uncaught IndexError: Invalid or unexpected index";

#[derive(Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
    line: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Span { start, end, line }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    source: Vec<u8>,
    source_length: usize,
    pos: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(source: Vec<u8>) -> Self {
        let source_length = source.len();
        Self {
            source,
            source_length,
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> (Vec<Token>, Vec<Span>) {
        let mut tokens: Vec<Token> = vec![];
        let mut spans: Vec<Span> = vec![];

        let mut my_push_token = |start, end, line, token| {
            spans.push(Span::new(start, end, line));
            tokens.push(token);
        };

        while self.pos < self.source_length {
            match self.peek_byte() {
                b'\n' => {
                    self.pos += 1;
                    self.line += 1;
                }
                b'+' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::PlusEqual
                        }
                        _ => Token::Plus,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'-' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::MinusEqual
                        }
                        b'>' => {
                            self.pos += 1;
                            length += 1;
                            Token::SingleArrow
                        }
                        // numeric
                        b if (b >= b'0' && b <= b'9') => {
                            let (_token, _length) = self.parse_int();
                            length += _length;
                            _token
                        }
                        _ => Token::Minus,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'*' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::MultiplyEqual
                        }
                        _ => Token::Asterisk,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'/' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::DivideEqual
                        }
                        b'/' => {
                            // handle line comments
                            self.pos += 1;
                            length += 1;
                            let (_token, _length) = self.parse_line_comment();
                            length += _length;
                            _token
                        }
                        b'*' => {
                            // handle block comments
                            self.pos += 1;
                            length += 1;
                            let (_token, _length) = self.parse_block_comment();
                            length += _length;
                            _token
                        }
                        _ => Token::Solidus,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'\\' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::ReverseSolidus);
                }
                b'&' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::AndEqual
                        }
                        _ => Token::Ampersand,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'@' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::AtMark);
                }
                b'#' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Hashtag);
                }
                b'$' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Dollar);
                }
                b'%' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Percent);
                }
                b'~' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Tilde);
                }
                b'^' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::XorEqual
                        }
                        _ => Token::Circumflex,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'|' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::OrEqual
                        }
                        _ => Token::VerticalLine,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'\'' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Apostrophe);
                }
                b'(' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::LeftParen);
                }
                b')' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::RightParen);
                }
                b'[' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::LeftSquareBracket);
                }
                b']' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::RightSquareBracket);
                }
                b'{' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::LeftCurlyBracket);
                }
                b'}' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::RightCurlyBracket);
                }
                b'<' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::LessEqual
                        }
                        _ => Token::LessThan,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'>' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::GreaterEqual
                        }
                        _ => Token::GreaterThan,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'.' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::FullStop);
                }
                b',' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Comma);
                }
                b'!' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::NotEqual
                        }
                        _ => Token::Exclamation,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                b'?' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Question);
                }
                b':' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Colon);
                }
                b';' => {
                    self.pos += 1;
                    my_push_token(self.pos - 1, self.pos, self.line, Token::Semicolon);
                }
                b'=' => {
                    self.pos += 1;
                    let mut length = 1;
                    let token = match self.peek_byte() {
                        b'=' => {
                            self.pos += 1;
                            length += 1;
                            Token::Equal
                        }
                        _ => Token::Assign,
                    };
                    my_push_token(self.pos - length, self.pos, self.line, token);
                }
                // string
                b'"' => {
                    self.pos += 1;
                    let mut length = 1;
                    let (_token, _length) = self.parse_string();
                    length += _length;
                    my_push_token(self.pos - length, self.pos, self.line, _token);
                }
                // numeric
                b if (b >= b'0' && b <= b'9') => {
                    let (_token, _length) = self.parse_int();
                    my_push_token(self.pos - _length, self.pos, self.line, _token);
                }
                // identifier
                b if (b == b'_') || (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') => {
                    let (_token, _length) = self.parse_identifier();
                    my_push_token(self.pos - _length, self.pos, self.line, _token);
                }
                // whitespace
                b if (b == b' ') || (b == b'\r') || (b == b'\t') => self.pos += 1,
                _ => panic!("{SYNTAX_ERROR_MSG} `{}`", self.peek_byte() as char),
            }
        }

        (tokens, spans)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source_length
    }

    fn peek_byte(&self) -> u8 {
        let byte: u8;
        if self.is_eof() {
            byte = b' ';
        } else {
            byte = *self.source.get(self.pos).expect(INDEX_ERROR_MSG);
        }
        byte
    }

    fn next_byte(&mut self) -> u8 {
        let byte: u8;
        if self.is_eof() {
            byte = b' ';
        } else {
            byte = *self.source.get(self.pos).expect(INDEX_ERROR_MSG);
            self.pos += 1;
        }
        byte
    }

    fn parse_line_comment(&mut self) -> (Token, usize) {
        todo!()
    }

    fn parse_block_comment(&self) -> (Token, usize) {
        todo!()
    }

    fn parse_string(&self) -> (Token, usize) {
        todo!()
    }

    fn parse_int(&self) -> (Token, usize) {
        todo!()
    }

    fn parse_identifier(&self) -> (Token, usize) {
        todo!()
    }

    fn parse_whitespace(&self) -> (Token, usize) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "";
        let output = Tokenizer::new(input.into()).tokenize();
        assert_eq!((vec![], vec![]), output)
    }
}
