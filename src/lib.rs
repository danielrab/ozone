pub mod tokenizer;
use tokenizer::BracketType;
use tokenizer::parse as source_to_tokens;

use std::fmt::Display;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use colored::Colorize;

#[derive(Debug)]
enum Token<'a> {
    String(&'a str),
    Operator(&'a str),
    Number(BigDecimal),
    Identifier(&'a str),
    Bracketed(BracketType, Vec<tokenizer::Token<'a>>),
}
#[derive(Debug, Default)]
struct Statement<'a>(Vec<Token<'a>>);

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let repr = match self {
            String(s) => format!("{s:?}").truecolor(255, 128, 0),
            Operator(o) => o.truecolor(128, 128, 128),
            Bracketed(bracket_type, _) => match bracket_type {
                BracketType::Curly => "{...}",
                BracketType::Round => "(...)",
                BracketType::Square => "[...]",
            }.to_owned().white(),
            Number(i) => i.to_string().yellow(),
            Identifier(id) => id.blue(),
        };
        write!(f, "{repr}")
    }
}

#[derive(Debug)]
pub enum Error {
    MismatchedClosingParentheses(String),
    LexingError,
}

fn tokens_to_statements(tokens: Vec<tokenizer::Token>) -> Result<Vec<Statement>, Error> {
    let mut statements = vec![Statement::default()];
    for token in tokens {
        let token = match token {
            tokenizer::Token::String(content) => Token::String(content),
            tokenizer::Token::Operator(content) => Token::Operator(content),
            tokenizer::Token::Bracketed((bracket_type, content)) => Token::Bracketed(bracket_type, content),
            tokenizer::Token::Number(content) => Token::Number(content),
            tokenizer::Token::Identifier(content) => Token::Identifier(content),
            tokenizer::Token::Comment(_) => continue,
            tokenizer::Token::Semicolon => {statements.push(Statement::default()); continue},
            tokenizer::Token::ClosingBracket(content) => return Err(Error::MismatchedClosingParentheses(content.to_owned())),
            tokenizer::Token::Error => return Err(Error::LexingError),
        };
        let len = statements.len();
        statements[len-1].0.push(token);
    }
    Ok(statements)
}

fn statements_to_expression(statements: Vec<Statement>) -> Result<(), Error> {
    for statement in statements {
        for token in statement.0 {
            print!("{token} ");
        }
        println!();
    }
    Ok(())
}

pub fn parse(source: String) -> Result<(), Error> {
    let tokens = source_to_tokens(&source);
    let statements = tokens_to_statements(tokens)?;
    let _ast = statements_to_expression(statements)?;
    Ok(())
}