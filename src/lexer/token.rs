use crate::lexer::kind::Kind;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: Kind, start: usize, end: usize) -> Self {
        Token {
            kind,
            span: Span { start, end },
        }
    }
}
