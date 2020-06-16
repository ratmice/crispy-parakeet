use rowan;
mod lex;
// Bring in the trait.
use logos::Logos as _;
use rowan::NodeOrToken;

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

pub mod stuff {
  use rowan::{SyntaxKind, SmolStr};
  use std::rc::Rc;
  pub type Elem = (SyntaxKind, SmolStr);

  #[derive(Debug)]
  pub enum NumOrExpr {
    One(Elem),
    Three(Rc<NumOrExpr>, Rc<NumOrExpr>, Rc<NumOrExpr>),
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
type SyntaxNode = rowan::SyntaxNode<Lang>;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<Lang>;
#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

impl rowan::Language for Lang {
    type Kind = lex::Token;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= lex::Token::Root as u16);
        unsafe { std::mem::transmute::<u16, lex::Token>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

fn print(indent: usize, element: SyntaxElement) {
    let kind: lex::Token = element.kind().into();
    print!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        }

        NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
    }
}

fn main() -> Result<(), error::MainError> {
    let s = "1 + 2 * 3 - 4";
    let lexer = LexWrap {
        lexer: lex::Token::lexer(s),
    };

    let mut builder = rowan::GreenNodeBuilder::new();
    builder.start_node(lex::Token::Root.into());
    let foo = parser::ExprParser::new().parse(&mut builder, lexer);
    println!("{:?}", foo.unwrap());
    builder.finish_node();
    let ast = SyntaxNode::new_root(builder.finish());
    print(0, ast.into());
    Ok(())
}
