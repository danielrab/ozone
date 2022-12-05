use logos::Logos;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum BracketType {
    Round,
    Square,
    Curly,
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    #[regex("#*\"", string_lexer)]
    String(&'a str),

    #[regex("//.*")]
    #[token("(*", comment_lexer)]
    Comment(&'a str),

    #[token(";")]
    Semicolon,

    #[regex(r"[!@#$%^&*+=\\|'/?,.<>:-]+")]
    Operator(&'a str),

    #[token("(", |lex| {(BracketType::Round, parse_inner(lex, ")"))})]
    #[token("[", |lex| {(BracketType::Square, parse_inner(lex, "]"))})]
    #[token("{", |lex| {(BracketType::Curly, parse_inner(lex, "}"))})]
    Bracketed((BracketType, Vec<Token<'a>>)),

    // this should never actually end up in the parsed token stream
    #[regex(r"[)}\]]")]
    ClosingBracket(&'a str),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Number(BigDecimal),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn parse_inner<'a>(lex: &mut logos::Lexer<'a, Token<'a>>, closing_bracket: &str) -> Vec<Token<'a>> {
    lex.take_while(|t| t != &Token::ClosingBracket(closing_bracket))
        .collect()
}

pub fn parse(source: &str) -> Vec<Token> {
    parse_inner(&mut Token::lexer(source), ")")
}

fn string_lexer<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) -> Result<&'a str, ()> {
    let tag = &lex.slice()[..lex.slice().len() - 1];
    let rem = lex.remainder();
    let mut escaping = false;

    for (i, &current) in rem.as_bytes().iter().enumerate() {
        if current == b'"' && !escaping && rem[i + 1..].starts_with(tag) {
            lex.bump(i + 1 + tag.len());
            return Ok(&rem[..i]);
        }
        escaping = current == b'\\' && !escaping;
    }
    Err(())
}

fn comment_lexer<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) -> Result<&'a str, ()> {
    let rem = lex.remainder();
    let mut nesting = 0;

    for (i, current) in rem.as_bytes().windows(2).enumerate() {
        match current {
            b"(*" => nesting += 1,
            b"*)" if nesting > 0 => nesting -= 1,
            b"*)" => {
                lex.bump(i + 2);
                return Ok(&rem[..i]);
            }
            _ => {}
        }
    }

    Err(())
}

#[cfg(test)]
mod test {
    use super::Token;
    use logos::Logos;
    #[test]
    fn comments() {
        check_lexer("(**)", &[Token::Comment("")]);
        check_lexer("(***)", &[Token::Comment("*")]);
        check_lexer("(*(**)*)", &[Token::Comment("(**)")]);
        check_lexer("(**)(**)", &[Token::Comment(""), Token::Comment("")]);
        check_lexer("(*привет*)", &[Token::Comment("привет")]);
    }
    fn check_lexer(input: &str, output: &[Token]) {
        let lex = Token::lexer(input);
        let tokens: Vec<Token> = lex.collect();
        assert_eq!(tokens, output, "{input}")
    }
}
