#[derive(Debug, PartialEq, Clone)]
pub enum TokenTypes {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    SLASH,
    ASTERISK,
    BANG,
    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LT,
    GT,

    // Keywords
    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub tokentype: TokenTypes,
    pub literal: String,
}
