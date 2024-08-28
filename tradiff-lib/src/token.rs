use std::num::ParseIntError;

use logos::Logos;


#[derive(Clone, Debug, PartialEq, Default)]
pub enum LexError {
    IntegerOverflow,
    InvalidDigit,
    InvalidInteger(String),
    InvalidToken,
    #[default]
    Unspecified,
}

impl From<ParseIntError> for LexError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexError::IntegerOverflow,
            InvalidDigit => LexError::InvalidDigit,
            error => LexError::InvalidInteger(format!("{error:?}")),
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", error = LexError)]
pub enum Token {

    #[regex(r"@-?[0-9]+", |lex| lex.slice()[1..].parse())] // [0-9]+ because that's what weidu does
    Id(i64),

    #[token("=")]
    Equal,

    #[token("^")]
    OperatorConcat,

    #[regex(
        r"~~~~~([^~]*|[^~]*~[^~]*|[^~]*~~[^~]*|[^~]*~~~[^~]*|[^~]*~~~~[^~]*)~~~~~",
        |lex| { let s = &lex.slice(); s[5..s.len()-5].to_string() }
    )]
    FiveTildesString(String),
    #[regex(r"~[^~]*~", |lex| { let s = &lex.slice(); s[1..s.len()-1].to_string() })]
    TildeString(String),
    #[regex(r#""[^"]*""#, |lex| { let s = &lex.slice(); s[1..s.len()-1].to_string() })]
    DoubleQuoteString(String),
    #[regex(r"%[^%]*%", |lex| { let s = &lex.slice(); s[1..s.len()-1].to_string() })]
    PercentString(String),

    #[regex("//[^\n]*\n", |lex| { let s = &lex.slice(); s[2..s.len() - 1].to_string() })]
    EndOfLineComment(String),
    #[regex(r"/\*([^\*]|\*[^/])*\*/", |lex| { let s = &lex.slice(); s[2..s.len() - 2].to_string() })]
    EnclosedComment(String),

    #[regex(r"\[[^\]]+\]", |lex| { let s = &lex.slice(); s[1..s.len()-1].to_string() })]
    SoundRef(String),

    #[regex(r"#-?[0-9]+", |lex| lex.slice()[1..].parse())]
    TlkRef(u64),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
