use std::{fmt, rc::Rc};

#[derive(PartialEq)]
pub enum Token {
    Int(i64),
    String(Rc<String>),
    Bool(bool),
    Identifier(Rc<String>),
    LineComment(Rc<String>),
    BlockComment(Rc<String>),

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

    // Double operator
    Equal,          // `==`
    LessEqual,      // `<=`
    GreaterEqual,   // `>=`
    NotEqual,       // `!=`
    PlusAssign,     // `+=`
    MinusAssign,    // `-=
    MultiplyAssign, // `*=`
    DivideAssign,   // `/=`
    SurplusAssign,  // `%=`
    XorAssign,      // `^=`
    AndAssign,      // `&=`
    OrAssign,       // `|=`
    SingleArrow,    // `->`
    DoubleArrow,    // `=>`
    LeftShift,      // `<<`
    RightShift,     // `>>`

    // Triple operator
    LeftShiftAssign,  // `<<=`
    RightShiftAssign, // `>>=`
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String;
        let str = match self {
            Token::Int(value) => {
                s = format!("Int(\"{value}\")");
                s.as_str()
            }
            Token::String(value) => {
                s = format!("String(\"{value}\")");
                s.as_str()
            }
            Token::Bool(value) => {
                s = format!("Bool(\"{value}\")");
                s.as_str()
            }
            Token::Identifier(value) => {
                s = format!("Identifier(\"{value}\")");
                s.as_str()
            }
            Token::LineComment(value) => {
                s = format!("LineComment(\"{value}\")");
                s.as_str()
            }
            Token::BlockComment(value) => {
                s = format!("BlockComment(\"{value}\")");
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
            Token::PlusAssign => "+=",
            Token::MinusAssign => "-=",
            Token::MultiplyAssign => "*=",
            Token::DivideAssign => "/=",
            Token::SurplusAssign => "^=",
            Token::XorAssign => "^=",
            Token::AndAssign => "&",
            Token::OrAssign => "|",
            Token::SingleArrow => "->",
            Token::DoubleArrow => "=>",
            Token::LeftShift => "<<",
            Token::RightShift => ">>",

            Token::LeftShiftAssign => "<<=",
            Token::RightShiftAssign => ">>=",
        };
        write!(f, "{str}")
    }
}
