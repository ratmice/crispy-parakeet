use sorbus;
mod lex;
// Bring in the trait.
use logos::Logos as _;
use sorbus::NodeOrToken;
use std::fmt;

mod parser {
    #![allow(clippy::all)]
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(calc);
    pub use calc::*;
}

mod error {
    #[derive(Debug)]
    pub enum MainError {
        IO(std::io::Error),
    }

    impl<'a> From<std::io::Error> for MainError {
        fn from(it: std::io::Error) -> Self {
            MainError::IO(it)
        }
    }
}

struct LexWrap<'a> {
    lexer: logos::Lexer<'a, lex::Token>,
}
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl Iterator for LexWrap<'_> {
    type Item = Spanned<lex::TokenWrap, usize, lex::LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            // Not dealing with whitespace in the parser yet
            Some(lex::Token::Whitespace) => {
                self.next()
            }
            Some(token) => {
                let r = self.lexer.span();
                Some(Ok((
                    r.start,
                    lex::TokenWrap::Token {
                        token,
                        string: self.lexer.slice().into(),
                    },
                    r.end,
                )))
            },

            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Lang {}
type SyntaxNode = sorbus::green::Node;
#[allow(unused)]
type SyntaxToken = sorbus::green::Token;
#[allow(unused)]
type SyntaxElement = sorbus::NodeOrToken<SyntaxNode, SyntaxToken>;

// From Rowan but not implemented in sorbus.
pub trait Language: Sized + Clone + Copy + fmt::Debug + Eq + Ord + std::hash::Hash {
    type Kind: fmt::Debug;

    fn kind_from_raw(raw: sorbus::Kind) -> Self::Kind;
    fn kind_to_raw(kind: Self::Kind) -> sorbus::Kind;
}

// Shamelessly stolen/slightly modified from the sorbus pratt example.
fn to_sexpr(node: &SyntaxNode) -> String {
    fn display<'a>(
        el: NodeOrToken<&'a SyntaxNode, &'a SyntaxToken>,
        f: &'a mut dyn fmt::Write,
    ) -> fmt::Result {
        match el {
            NodeOrToken::Token(token) => write!(f, "{}", token.text())?,
            NodeOrToken::Node(node) => {
                let children_of_interest: Vec<_> =
                    node.children().filter(|el| el.kind() != lex::Token::Whitespace.into()).collect();
                if children_of_interest.len() == 1 {
                    display(children_of_interest[0].as_deref(), f)?;
                } else {
                    f.write_str("(")?;
                    for op in children_of_interest.iter() {
                        display(op.as_deref(), f)?;
                    }
                    f.write_str(")")?;
                }
            }
        }
        Ok(())
    }
    let mut s = String::new();
    display(NodeOrToken::Node(node), &mut s).unwrap();
    s
}


fn main() -> Result<(), error::MainError> {
    let s = "1 + 2 * 3 - 4";
    let lexer = LexWrap {
        lexer: lex::Token::lexer(s),
    };

    let mut builder = sorbus::green::TreeBuilder::new();
    builder.start_node(lex::Token::Root.into());
    let foo = parser::ExprParser::new().parse(&mut builder, lexer);
    println!("{:?}", foo.unwrap());
    builder.finish_node();
    let ast = builder.finish();
    println!("{}", to_sexpr(&ast));
    println!("{:#?}", ast);
    Ok(())
}
