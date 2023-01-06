use crate::{ErrorKind, Error};
use crate::util::Token;
use crate::tokenizer::{Bracket, BracketDirection, BracketType, RawToken, Terminator};

#[derive(Debug)]
pub enum BasicToken<'a> {
    String(&'a str),
    Operator(&'a str),
    Identifier(&'a str),
    Bracketed(BracketType, TokenStream<'a>),
    Terminator(Terminator),
    Dot,
}

impl std::fmt::Display for BasicToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s:?}"),
            Self::Operator(s) => write!(f, "{s}"),
            Self::Identifier(s) => write!(f, "{s}"),
            Self::Bracketed(BracketType::Curly, _) => write!(f, "{{...}}"),
            Self::Bracketed(BracketType::Round, _) => write!(f, "(...)"),
            Self::Bracketed(BracketType::Square, _) => write!(f, "[...]"),
            Self::Terminator(s) => write!(f, "{s}"),
            Self::Dot => write!(f, "."),
        }
    }
}

pub type TokenStream<'a> = Vec<Token<BasicToken<'a>>>;

pub fn group_parentheses<'a>(tokens: &'a [Token<RawToken<'a>>]) -> Result<TokenStream<'a>, Error> {
    let mut res = vec![];
    let mut current_bracket = None;
    let mut depth = 0;
    let mut bracket_start = 0;
    for (i, Token { token, span }) in tokens.iter().enumerate() {
        if depth > 0 && !matches!(token, RawToken::Bracket(..)) {
            continue;
        }
        let token = match *token {
            RawToken::String(content) => BasicToken::String(content),
            RawToken::Operator(content) => BasicToken::Operator(content),
            RawToken::Identifier(content) => BasicToken::Identifier(content),
            RawToken::SpecialCharacter(c) => BasicToken::Terminator(c),
            RawToken::Dot => BasicToken::Dot,
            RawToken::Bracket(Bracket(t, BracketDirection::Opening)) => {
                if depth == 0 {
                    bracket_start = i;
                    current_bracket = Some(t);
                }
                if current_bracket == Some(t) {
                    depth += 1;
                }
                continue;
            }
            RawToken::Bracket(Bracket(t, BracketDirection::Closing)) => {
                if Some(t) != current_bracket {
                    return Err(Error {kind: ErrorKind::MismatchedClosingParentheses, span: span.clone()});
                }
                depth -= 1;
                if depth > 0 {
                    continue;
                }
                current_bracket = None;
                res.push(Token {
                    token: BasicToken::Bracketed(t, group_parentheses(&tokens[bracket_start + 1..i])?),
                    span: tokens[bracket_start].span.start..tokens[i].span.start,
                });
                continue;
            }
            RawToken::Error => return Err(Error {kind: ErrorKind::LexingError, span: span.clone()}),
        };
        res.push(Token {
            token,
            span: span.clone(),
        });
    }
    Ok(res)
}

