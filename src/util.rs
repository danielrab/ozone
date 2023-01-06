use std::fmt::Debug;
use std::ops::Range;

#[derive(Debug)]
pub struct Token<Kind: Debug> {
    pub token: Kind,
    pub span: Range<usize>,
}
