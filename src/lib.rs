pub mod tokenizer;
use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TokenKind {
    String,
    Character,
    Identifier,
    Operator,
    Brace,
}
#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Range<usize>,
    pub content: &'a str,
}
impl<'a> TryFrom<tokenizer::Token<'a>> for Token<'a> {
    type Error = ();
    fn try_from(token: tokenizer::Token<'a>) -> Result<Self, Self::Error> {
        use tokenizer::TokenKind as TK;
        let kind = match token.kind {
            TK::Comment => return Err(()),
            TK::Whitespace => return Err(()),
            TK::Brace => TokenKind::Brace,
            TK::Character => TokenKind::Character,
            TK::Identifier => TokenKind::Identifier,
            TK::Operator => TokenKind::Operator,
            TK::String => TokenKind::String
        };
        Ok(Self { kind, span: token.span, content: token.content })
    }
}

pub fn parse(source: &str) -> Result<Vec<Token>, tokenizer::Error> {
    let all_tokens = tokenizer::parse(source)?;
    let meaningful_tokens = all_tokens
        .into_iter()
        .flat_map(Token::try_from)
        .collect();
    Ok(meaningful_tokens)
}
