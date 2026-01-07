pub struct Lexer {
    source: String,
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Constant(String),
    Symbol(Symbol),
    IntLiteral(String),
}

#[derive(Debug)]
pub enum Keyword {
    Int,
    Return,
}

#[derive(Debug)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
