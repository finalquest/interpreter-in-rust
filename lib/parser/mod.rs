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
            match statament {
                Some(st) => statemets.push(st),
                None => ()
            }
            self.next_token();
        }

        return statemets;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.cur_token.tokentype == TokenTypes::LET {
            return self.parse_let();
        }

        None
    }

    fn parse_let(&mut self) -> Option<Statement> {
        match self.peek_token.tokentype {
            TokenTypes::IDENT(_) => self.next_token(),
            _ => return None
        }

        let ident = Ident(self.cur_token.literal.to_string());

        if !self.expect_peek(TokenTypes::ASSIGN) {
            return None
        }

        while !self.cur_token_is(TokenTypes::SEMICOLON) {
            self.next_token();
        }

        let lt = Statement::LetStatement(ident,Expression::Ident(Ident("".to_string())));

        return Some(lt);
    }

    fn expect_peek(&mut self, token_type: TokenTypes) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        return false;
    }

    fn cur_token_is(&mut self, token_type: TokenTypes) -> bool {
        return self.cur_token.tokentype == token_type;
    }

    fn peek_token_is(&mut self, token_type: TokenTypes) -> bool {
        return self.peek_token.tokentype == token_type;
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
            let foobar = 838383;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();


        assert_eq!(program.len(), 3);

        assert_eq!(vec![
                Statement::LetStatement(Ident(String::from("x")), Expression::Ident(Ident(String::from("")))),
                Statement::LetStatement(Ident(String::from("y")), Expression::Ident(Ident(String::from("")))),
                Statement::LetStatement(
                    Ident(String::from("foobar")),
                    Expression::Ident(Ident(String::from("")))
                ),
            ], program);
    }
}
