use std::{ops::Range, str::from_utf8};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TokenKind {
    String,
    Character,
    Identifier,
    Operator,
    Whitespace,
    Comment,
    Brace,
}
#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Range<usize>,
    pub content: &'a str,
}
impl From<u8> for TokenKind {
    fn from(c: u8) -> Self {
        match c {
            b'"' | b'#' => Self::String,
            b'\'' => Self::Character,
            b'/' => Self::Comment,
            b'[' | b']' | b'{' | b'}' | b'(' | b')' => Self::Brace,
            0..=31 | b' ' | 127 => Self::Whitespace,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | 128.. => Self::Identifier,
            _ => Self::Operator,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnfinishedToken(TokenKind),
}

fn extract_count<'a>(source: &mut &'a [u8], count: usize) -> Option<&'a [u8]> {
    let res = &source.get(..count)?;
    *source = &source[count..];
    Some(res)
}

fn extract_to_end<'a>(source: &mut &'a [u8]) -> &'a [u8] {
    let res = &source[..];
    *source = Default::default();
    res
}

fn extract_until<'a>(source: &mut &'a [u8], ending: &[u8], from: usize) -> Option<&'a [u8]> {
    source
        .windows(ending.len())
        .skip(from)
        .position(|s| s == ending)
        .map(|p| p + from)
        .and_then(|p| extract_count(source, p + ending.len()))
}

fn extract_type<'a>(source: &mut &'a [u8]) -> &'a [u8] {
    source
        .iter()
        .position(|&c| TokenKind::from(c) != TokenKind::from(source[0]))
        .and_then(|p| extract_count(source, p))
        .unwrap_or_else(|| extract_to_end(source))
}

fn extract_string<'a>(source: &mut &'a [u8], tag: Option<&[u8]>) -> Option<&'a [u8]> {
    let mut position = 1;
    let tag = tag.map(|t| [t, b"#"].concat()).unwrap_or_default();

    loop {
        let remaining_source = &source.get(position..)?;
        let point_of_interest = remaining_source.iter().position(|c| b"\"\\|".contains(c))?;

        let char = remaining_source[point_of_interest];
        let remaining_source = &remaining_source[point_of_interest + 1..];
        position += point_of_interest + 1;

        let skip = match char {
            b'\\' => 1,
            b'|' => remaining_source.iter().position(|&c| c == b'|')? + 1,
            b'"' if remaining_source[..tag.len()] == tag => return extract_count(source, position+tag.len()),
            _ => 0,
        };
        position += skip;
    }
}

fn extract_token<'a>(source: &mut &'a [u8]) -> Result<(TokenKind, &'a [u8]), Error> {
    let kind = TokenKind::from(source[0]);
    let content = match source {
        [b'#', ..] => {
            let tag_end = source
                .iter()
                .position(|&c| c == b'"')
                .ok_or(Error::UnfinishedToken(kind))?;
            extract_string(source, Some(&source[1..tag_end])).ok_or(Error::UnfinishedToken(kind))?
        }
        [b'"', ..] => extract_string(source, None).ok_or(Error::UnfinishedToken(kind))?,
        [b'/', b'*', ..] => extract_until(source, b"*/", 2).ok_or(Error::UnfinishedToken(kind))?,
        [b'/', b'/', ..] => {
            extract_until(source, b"\n", 0).unwrap_or_else(|| extract_to_end(source))
        }
        [b'\'', b'|', ..] => extract_until(source, b"|", 2).ok_or(Error::UnfinishedToken(kind))?,
        [b'\'', b'\\', ..] => extract_count(source, 3).ok_or(Error::UnfinishedToken(kind))?,
        [b'\'', ..] => extract_count(source, 2).ok_or(Error::UnfinishedToken(kind))?,
        _ if kind == TokenKind::Brace => extract_count(source, 1).unwrap(),
        _ => extract_type(source),
    };
    Ok((kind, content))
}

pub fn parse(source: &str) -> Result<Vec<Token>, Error> {
    let mut source = source.as_bytes();
    let mut tokens = Vec::new();
    let mut position = 0;

    while !source.is_empty() {
        let (kind, content) = extract_token(&mut source)?;
        let content = from_utf8(content).expect("token isn't utf-8");
        let span = position..position + content.len();

        position += content.len();
        tokens.push(Token {
            kind,
            content,
            span,
        });
    }
    Ok(tokens)
}
