use logos::Logos;
use rowan;
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

    Operation1,
    Operation2,
    Operation3,
    Uhh,
    // Parent of all the above.
    Root,
}

pub type LexicalError = Range<usize>;

impl From<Token> for rowan::SyntaxKind {
    fn from(kind: Token) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone)]
pub enum TokenWrap {
    Token {
        token: Token,
        string: rowan::SmolStr,
    },
}
