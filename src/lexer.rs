use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

#[derive(Debug)]
pub enum Token {
    // Keywords
    Int,
    Return,

    #[allow(unused)]
    Identifier(String),

    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,

    // Literals
    #[allow(unused)]
    IntLiteral(String),
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace before each token
        self.skip_whitespace();

        // Get the next character
        let c = self.advance()?;

        // Match single-character tokens
        let token = match c {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ';' => Token::Semicolon,
            // Delegate to complex token handler for everything else
            _ => return self.lex_complex_token(c),
        };

        Some(token)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    /// Peek at the next character without consuming it
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Consume and return the next character
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn lex_complex_token(&mut self, first: char) -> Option<Token> {
        // Check if it's an identifier or keyword
        if first.is_alphabetic() || first == '_' {
            return Some(self.lex_identifier_or_keyword(first));
        } else if first.is_ascii_digit() {
            return Some(self.lex_numeric_literal(first));
        }

        // For now, panic on unrecognized characters
        panic!("Unrecognized token: {first}");
    }

    fn lex_identifier_or_keyword(&mut self, first: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first);

        // Continue collecting alphanumeric characters and underscores
        while let Some(&ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        match identifier.as_str() {
            "int" => Token::Int,
            "return" => Token::Return,
            _ => Token::Identifier(identifier),
        }
    }

    fn lex_numeric_literal(&mut self, first: char) -> Token {
        let mut literal = String::new();
        literal.push(first);

        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() {
                literal.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::IntLiteral(literal)
    }
}
