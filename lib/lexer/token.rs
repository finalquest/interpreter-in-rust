#[derive(Debug, PartialEq, Clone)]
pub enum TokenTypes {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

pub struct Token {
    pub tokentype: TokenTypes,
    pub literal: String,
}
