use logos::Logos;
use sorbus;
use smol_str::SmolStr;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum Token {
    #[regex(r"[ \t\n\f]+")]
    Whitespace = 0,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex(r"[0-9]+")]
    Number,

    #[error]
    Error,

    Oper,
    // Parent of all the above.
    Root,
}

pub type LexicalError = Range<usize>;

impl From<Token> for sorbus::Kind {
    fn from(kind: Token) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone)]
pub enum TokenWrap {
    Token {
        token: Token,
        string: SmolStr,
    },
}
