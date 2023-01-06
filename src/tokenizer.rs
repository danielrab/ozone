use logos::{Lexer, Logos};

use crate::util::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BracketType {
    Round,
    Square,
    Curly,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BracketDirection {
    Opening,
    Closing,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bracket(pub BracketType, pub BracketDirection);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Terminator {
    Semicolon,
    Comma,
}
impl std::fmt::Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semicolon => write!(f, ";"),
            Self::Comma => write!(f, ","),
        }
    }
}

#[derive(Logos, Debug, Clone, Copy)]
pub enum RawToken<'a> {
    #[regex("#*\"", string_lexer)]
    String(&'a str),

    #[regex(r"[~!@#$%^&*+=\\|/?><':-]+")]
    #[regex(r"`[a-zA-Z0-9_]+`")]
    Operator(&'a str),

    #[regex(r"[a-zA-Z0-9_]+")]
    Identifier(&'a str),

    #[token("(", |_| Bracket(BracketType::Round, BracketDirection::Opening))]
    #[token("[", |_| Bracket(BracketType::Square, BracketDirection::Opening))]
    #[token("{", |_| Bracket(BracketType::Curly, BracketDirection::Opening))]
    #[token(")", |_| Bracket(BracketType::Round, BracketDirection::Closing))]
    #[token("]", |_| Bracket(BracketType::Square, BracketDirection::Closing))]
    #[token("}", |_| Bracket(BracketType::Curly, BracketDirection::Closing))]
    Bracket(Bracket),

    #[token(".")]
    Dot,

    #[token(";", |_| Terminator::Semicolon)]
    #[token(",", |_| Terminator::Comma)]
    SpecialCharacter(Terminator),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn tokenize(source: &str) -> Vec<Token<RawToken>> {
    RawToken::lexer(source).spanned().map(|(token, span)| Token { token, span }).collect()
}

fn string_lexer<'a>(lex: &mut Lexer<'a, RawToken<'a>>) -> Result<&'a str, ()> {
    let tag = &lex.slice()[..lex.slice().len() - 1];
    let rem = lex.remainder();

    for (i, &current) in rem.as_bytes().iter().enumerate() {
        if current == b'"' && rem[i + 1..].starts_with(tag) {
            lex.bump(i + 1 + tag.len());
            return Ok(&rem[..i]);
        }
    }
    Err(())
}
