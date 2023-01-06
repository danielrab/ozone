mod lexer;
mod tokenizer;
mod util;

use std::ops::Range;

use tokenizer::{tokenize, Terminator};
use lexer::{group_parentheses, TokenStream, BasicToken};
use util::Token;

#[derive(Debug)]
pub enum ErrorKind {
    MismatchedClosingParentheses,
    LexingError,
    MismatchedTerminator,
}
#[allow(unused)]
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Range<usize>,
}

fn group_expressions(tokens: TokenStream) -> Result<(Terminator ,Vec<TokenStream>), Error> {
    let mut current_terminator = None;
    let mut expressions = vec![vec![]];
    for Token { token, span } in tokens {
        let BasicToken::Terminator(terminator) = token else {
            expressions.last_mut().unwrap().push(Token { token, span });
            continue;
        };
        current_terminator = Some(current_terminator.unwrap_or(terminator));
        if current_terminator != Some(terminator) {
            return Err(Error {kind: ErrorKind::MismatchedTerminator, span });
        }
        expressions.push(vec![]);
    }
    Ok((current_terminator.unwrap_or(Terminator::Semicolon), expressions))
}

pub fn parse(source: &str) -> Result<(), Error> {
    let tokens = tokenize(source);
    let tokens = group_parentheses(&tokens)?;
    let (terminator, expressions) = group_expressions(tokens)?;
    println!("{terminator:?}");
    for expression in expressions {
        for Token { token, span: _ } in expression {
            print!("{token} ")
        }
        println!();
    }
    Ok(())
}
