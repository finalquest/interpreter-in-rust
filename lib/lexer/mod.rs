pub mod token;

use crate::lexer::token::*;

#[derive(PartialEq, Clone, Debug)]
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
        self.skip_whitespaces();
        let tok =  match self.ch {
            '+' => Token {
                tokentype: TokenTypes::PLUS,
                literal: self.ch.to_string(),
            },
            '-' => Token {
                tokentype: TokenTypes::MINUS,
                literal: self.ch.to_string(),
            },
            '!' => 
                if self.look_ahead() == '=' {
                    self.read_char();
                    Token {
                        tokentype: TokenTypes::NOT_EQ,
                        literal: "!=".to_string(),
                    }
                } else {
                    Token {
                        tokentype: TokenTypes::BANG,
                        literal: self.ch.to_string(),
                    }
                },
            '/' => Token {
                tokentype: TokenTypes::SLASH,
                literal: self.ch.to_string(),
            },
            '*' => Token {
                tokentype: TokenTypes::ASTERISK,
                literal: self.ch.to_string(),
            },
            '<' => Token {
                tokentype: TokenTypes::LT,
                literal: self.ch.to_string(),
            },
            '>' => Token {
                tokentype: TokenTypes::GT,
                literal: self.ch.to_string(),
            },
            '=' => 
                if self.look_ahead() == '=' {
                    self.read_char();
                    Token {
                        tokentype: TokenTypes::EQ,
                        literal: "==".to_string(),
                    }
                } else {
                    Token {
                        tokentype: TokenTypes::ASSIGN,
                        literal: self.ch.to_string(),
                    }
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
            _ => { 
                if self.ch.is_ascii_alphabetic() {
                    let literal = self.read_identifier();
                    let token_type = Lexer::lookup_builtin(&literal);
                    return Token {
                        tokentype: token_type,
                        literal 
                    }
                }

                if self.ch.is_numeric() {
                    let literal = self.read_number();
                    return Token {
                        tokentype: TokenTypes::INT(literal.clone()),
                        literal: literal.to_string(),
                    }
                }
                
                Token {
                  tokentype: TokenTypes::ILLEGAL,
                  literal: self.ch.to_string(),
                }
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

    fn look_ahead(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();  
        }
        return self.input[pos..self.position].to_string();
    }

    fn read_number(&mut self) -> i64 {
        let pos = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        return self.input[pos..self.position].parse::<i64>().unwrap();
    }

    fn lookup_builtin(lit: &String) -> TokenTypes {
        match lit.as_str() {
            "fn" => TokenTypes::FUNCTION,
            "let" => TokenTypes::LET,
            "true" => TokenTypes::TRUE,
            "false" => TokenTypes::FALSE,
            "if" => TokenTypes::IF,
            "else" => TokenTypes::ELSE,
            "return" => TokenTypes::RETURN,
            _ => TokenTypes::IDENT(lit.to_string()),
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
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

    #[test]
    fn test_lexer_2() {
        let input = "
let five = 5;
let ten = 10;
   let add = fn(x, y) {
     x + y;
};
   let result = add(five, ten);
   !-/*5;
   5 < 10 > 5;
if (5 < 10) {
       return true;
   } else {
       return false;
}
10 == 10; 10 != 9;
";

        let mut lexer = Lexer::new(input);
        let tests = vec![
            TokenTypes::LET,
            TokenTypes::IDENT("five".to_string()),
            TokenTypes::ASSIGN,
            TokenTypes::INT(5),
            TokenTypes::SEMICOLON,
            TokenTypes::LET,
            TokenTypes::IDENT("ten".to_string()),
            TokenTypes::ASSIGN,
            TokenTypes::INT(10),
            TokenTypes::SEMICOLON,
            TokenTypes::LET,
            TokenTypes::IDENT("add".to_string()),
            TokenTypes::ASSIGN,
            TokenTypes::FUNCTION,
            TokenTypes::LPAREN,
            TokenTypes::IDENT("x".to_string()),
            TokenTypes::COMMA,
            TokenTypes::IDENT("y".to_string()),
            TokenTypes::RPAREN,
            TokenTypes::LBRACE,
            TokenTypes::IDENT("x".to_string()),
            TokenTypes::PLUS,
            TokenTypes::IDENT("y".to_string()),
            TokenTypes::SEMICOLON,
            TokenTypes::RBRACE,
            TokenTypes::SEMICOLON,
            TokenTypes::LET,
            TokenTypes::IDENT("result".to_string()),
            TokenTypes::ASSIGN,
            TokenTypes::IDENT("add".to_string()),
            TokenTypes::LPAREN,
            TokenTypes::IDENT("five".to_string()),
            TokenTypes::COMMA,
            TokenTypes::IDENT("ten".to_string()),
            TokenTypes::RPAREN,
            TokenTypes::SEMICOLON,
            TokenTypes::BANG,
            TokenTypes::MINUS,
            TokenTypes::SLASH,
            TokenTypes::ASTERISK,
            TokenTypes::INT(5),
            TokenTypes::SEMICOLON,
            TokenTypes::INT(5),
            TokenTypes::LT,
            TokenTypes::INT(10),
            TokenTypes::GT,
            TokenTypes::INT(5),
            TokenTypes::SEMICOLON,
            TokenTypes::IF,
            TokenTypes::LPAREN,
            TokenTypes::INT(5),
            TokenTypes::LT,
            TokenTypes::INT(10),
            TokenTypes::RPAREN,
            TokenTypes::LBRACE,
            TokenTypes::RETURN,
            TokenTypes::TRUE,
            TokenTypes::SEMICOLON,
            TokenTypes::RBRACE,
            TokenTypes::ELSE,
            TokenTypes::LBRACE,
            TokenTypes::RETURN,
            TokenTypes::FALSE,
            TokenTypes::SEMICOLON,
            TokenTypes::RBRACE,
            TokenTypes::INT(10),
            TokenTypes::EQ,
            TokenTypes::INT(10),
            TokenTypes::SEMICOLON,
            TokenTypes::INT(10),
            TokenTypes::NOT_EQ,
            TokenTypes::INT(9),
            TokenTypes::SEMICOLON,
            TokenTypes::EOF,
            
        ];
        for (_i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.tokentype, *tt);
        }

    }
}
