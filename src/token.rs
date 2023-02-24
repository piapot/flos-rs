use std::{fmt, rc::Rc};

#[allow(unused)]
#[derive(PartialEq)]
pub enum Token {
    // Type
    Int(i64),
    String(Rc<String>),
    Char(char),
    Bool(bool),
    Identifier(Rc<String>),
    Comment(Rc<String>),

    // KeyWord
    Let,
    Mut,
    Const,
    If,
    Elif,
    Else,
    For,
    Of,
    While,
    When,
    Case,
    And,
    Or,
    Fn,
    Break,
    Continue,
    Return,
    Trait,
    Ext,
    Impl,
    Enum,
    As,
    Export,

    // Single operator
    Plus,               // `+`
    Minus,              // `-`
    Asterisk,           // `*`
    Solidus,            // `/`
    ReverseSolidus,     // `\`
    Ampersand,          // `&`
    AtMark,             // `@`
    Hashtag,            // `#`
    Dollar,             // `$`
    Percent,            // `%`
    Tilde,              // `~`
    Circumflex,         // `^`
    VerticalLine,       // `|`
    Apostrophe,         // `'`
    LeftParen,          // `(`
    RightParen,         // `)`
    LeftSquareBracket,  // `[`
    RightSquareBracket, // `]`
    LeftCurlyBracket,   // `{`
    RightCurlyBracket,  // `{`
    LessThan,           // `<`
    GreaterThan,        // `>`
    FullStop,           // `.`
    Comma,              // `,`
    Exclamation,        // `!`
    Question,           // `?`
    Colon,              // `:`
    Semicolon,          // `;`
    Assign,             // `=`

    Equal,         // `==`
    LessEqual,     // `<=`
    GreaterEqual,  // `>=`
    NotEqual,      // `!=`
    PlusEqual,     // `+=`
    MinusEqual,    // `-=
    MultiplyEqual, // `*=`
    DivideEqual,   // `/=`
    SurplusEqual,  // `%=`
    XorEqual,      // `^=`
    AndEqual,      // `&=`
    OrEqual,       // `|=`
    SingleArrow,   // `->`
    DoubleArrow,   // `=>`
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String;
        let str = match self {
            Token::Int(value) => {
                s = format!("{value}");
                s.as_str()
            }
            Token::String(value) => {
                s = format!("{value}");
                s.as_str()
            }
            Token::Char(value) => {
                s = format!("{value}");
                s.as_str()
            }
            Token::Bool(value) => {
                s = format!("{value}");
                s.as_str()
            }
            Token::Identifier(value) => {
                s = format!("{value}");
                s.as_str()
            }
            Token::Comment(value) => {
                s = format!("{value}");
                s.as_str()
            }
            Token::Let => "let",
            Token::Mut => "mut",
            Token::Const => "const",
            Token::If => "if",
            Token::Elif => "elif",
            Token::Else => "else",
            Token::For => "for",
            Token::Of => "of",
            Token::While => "while",
            Token::When => "when",
            Token::Case => "case",
            Token::And => "and",
            Token::Or => "or",
            Token::Fn => "fn",
            Token::Break => "break",
            Token::Continue => "continue",
            Token::Return => "return",
            Token::Trait => "trait",
            Token::Ext => "ext",
            Token::Impl => "impl",
            Token::Enum => "enum",
            Token::As => "as",
            Token::Export => "export",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Solidus => "/",
            Token::ReverseSolidus => "\\",
            Token::Ampersand => "&",
            Token::AtMark => "@",
            Token::Hashtag => "#",
            Token::Dollar => "$",
            Token::Percent => "%",
            Token::Tilde => "~",
            Token::Circumflex => "^",
            Token::VerticalLine => "|",
            Token::Apostrophe => "'",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftSquareBracket => "[",
            Token::RightSquareBracket => "]",
            Token::LeftCurlyBracket => "{",
            Token::RightCurlyBracket => "{",
            Token::LessThan => "<",
            Token::GreaterThan => ">",
            Token::FullStop => ".",
            Token::Comma => ",",
            Token::Exclamation => "!",
            Token::Question => "?",
            Token::Colon => ":",
            Token::Semicolon => ";",
            Token::Assign => "=",

            Token::Equal => "==",
            Token::LessEqual => "<=",
            Token::GreaterEqual => ">=",
            Token::NotEqual => "!=",
            Token::PlusEqual => "+=",
            Token::MinusEqual => "-=",
            Token::MultiplyEqual => "*=",
            Token::DivideEqual => "/=",
            Token::SurplusEqual => "^=",
            Token::XorEqual => "^=",
            Token::AndEqual => "&",
            Token::OrEqual => "|",
            Token::SingleArrow => "->",
            Token::DoubleArrow => "=>",
        };
        write!(f, "{str}")
    }
}
