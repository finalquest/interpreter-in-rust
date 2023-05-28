pub mod token;

use crate::lexer::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,

    _private:()
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
            _private: ()
        };
        lexer.read_char();
        lexer
    }
    
    pub fn next_token(&mut self) -> Token {
        let tok =  match self.ch {
            '=' => Token {
                tokentype: TokenTypes::ASSIGN,
                literal: self.ch.to_string(),
            },
            ';' => Token {
                tokentype: TokenTypes::SEMICOLON,
                literal: self.ch.to_string(),
            },
            '(' => Token {
                tokentype: TokenTypes::LPAREN,
                literal: self.ch.to_string(),
            },
            ')' => Token {
                tokentype: TokenTypes::RPAREN,
                literal: self.ch.to_string(),
            },
            ',' => Token {
                tokentype: TokenTypes::COMMA,
                literal: self.ch.to_string(),
            },
            '+' => Token {
                tokentype: TokenTypes::PLUS,
                literal: self.ch.to_string(),
            },
            '{' => Token {
                tokentype: TokenTypes::LBRACE,
                literal: self.ch.to_string(),
            },
            '}' => Token {
                tokentype: TokenTypes::RBRACE,
                literal: self.ch.to_string(),
            },
            '\0' => Token {
                tokentype: TokenTypes::EOF,
                literal: "".to_string(),
            },
            _ => Token {
                tokentype: TokenTypes::ILLEGAL,
                literal: self.ch.to_string(),
            },
        };
        self.read_char();
        return tok;
    } 
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);
        let tests = vec![
            TokenTypes::ASSIGN,
            TokenTypes::PLUS,
            TokenTypes::LPAREN,
            TokenTypes::RPAREN,
            TokenTypes::LBRACE,
            TokenTypes::RBRACE,
            TokenTypes::COMMA,
            TokenTypes::SEMICOLON,
            TokenTypes::EOF,
        ];
        for (_i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.tokentype, *tt);
        }

    }
}
