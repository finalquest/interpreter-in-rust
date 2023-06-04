use crate::lexer::*;
use crate::lexer::token::*;
use crate::ast::*;
use crate::parser::Statement;

#[derive(PartialEq, Clone, Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    _private: (),
 }


impl Parser {
    pub fn new(input: Lexer) -> Self {

        let mut parser = Self {
            _private:(),
            cur_token: Token{literal: "".to_string(), tokentype:TokenTypes::EOF},
            peek_token: Token{literal: "".to_string(), tokentype:TokenTypes::EOF},
            lexer: input,
        };

        parser.next_token();
        parser.next_token();
    
        return parser;
    } 

    pub fn next_token(&mut self) {
        self.cur_token=self.peek_token.clone();
        self.peek_token=self.lexer.next_token();
    }
    pub fn parse_program(&mut self) -> Program {
        let mut statemets = vec![];

        while self.cur_token.tokentype != TokenTypes::EOF {
            let statament = self.parse_statement();
        }

        return statemets;
    }

    fn parse_statement(&mut self) -> Statement {
        if self.cur_token.tokentype == TokenTypes::LET {
            return self.parse_let();
        }
    }

    fn parse_let(&mut self) -> Statement {
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_let() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();


        let test = vec!["x","y","foobar"];
        assert_eq!(program.len(), 3);
    }
}
