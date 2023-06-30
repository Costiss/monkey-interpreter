use crate::ast::{Expression, Node, Statement};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    EQ,
    NOTEQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPARAN,
    RPARAN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new_token(token_type: TokenType, literal: String) -> Token {
        return Token {
            token_type,
            literal,
        };
    }

    pub fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    pub fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }
}

pub struct LetStatement<'a> {
    pub token: Token, //TokenType.LET
    pub name: &'a Identifier,
    pub value: dyn Expression,
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for LetStatement<'_> {
    fn statements(&self) {
        todo!()
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Expression for Identifier {
    fn expression(self) {
        todo!()
    }
}
