use crate::lexer::*;
use crate::lexer::token::*;
use crate::ast::*;
use crate::parser::Statement;

#[derive(Clone, Debug)]
pub enum ParseErrorType {
    UnexpectedToken,
}

#[derive(Clone, Debug)]
pub struct ParseError {
    error_type: ParseErrorType,
    msg: String,
}

impl ParseError {
    fn new(e_type: ParseErrorType, msg: String) -> Self {
        ParseError { error_type: e_type, msg }
    }
}

#[derive(Clone, Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
    _private: (),
 }


impl Parser {
    pub fn new(input: Lexer) -> Self {

        let mut parser = Self {
            _private:(),
            cur_token: Token{literal: "".to_string(), tokentype:TokenTypes::EOF},
            peek_token: Token{literal: "".to_string(), tokentype:TokenTypes::EOF},
            lexer: input,
            errors: vec![]
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
        match self.cur_token.tokentype {
            TokenTypes::LET => return self.parse_let(),
            TokenTypes::RETURN => return self.parse_return(),
            _ => return self.parse_expression_statement()
        };
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

        let lt = Statement::Let(ident,Expression::Ident(Ident("".to_string())));

        return Some(lt);
    }

    fn parse_return(&mut self) -> Option<Statement> {
        println!("parse_return");
        self.next_token();

        while !self.cur_token_is(TokenTypes::SEMICOLON) {
            self.next_token();
        }

        return Some(Statement::Return(Expression::Ident(Ident("".to_string()))))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_exression(Precedence::Lowest);

        if self.peek_token_is(&TokenTypes::SEMICOLON) {
            self.next_token();
        }

        return Some(Statement::Expression(expression.unwrap()));
    }

    fn parse_exression(&mut self, _precedence: Precedence) -> Option<Expression> {

        match self.cur_token.tokentype {
            TokenTypes::IDENT(_) => return Some(Expression::Ident(Ident(self.cur_token.literal.to_string()))),
            TokenTypes::INT(ref mut int) => return Some(Expression::IntegerLiteral(int.clone())),
            TokenTypes::BANG | TokenTypes::MINUS => return self.parse_prefix_expression(),
            _ => return None
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prefix = match self.cur_token.tokentype {
            TokenTypes::BANG => Prefix::Not,
            TokenTypes::MINUS => Prefix::Minus,
            _ => return None
        };

        self.next_token();
        let right = self.parse_exression(Precedence::Prefix);
        match right {
            Some(r) => return Some(Expression::Prefix(prefix, Box::new(r))),
            None => return None
        }
    }

    fn expect_peek(&mut self, token_type: TokenTypes) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            return true;
        }
        self.peek_error(token_type);
        return false;
    }

    fn cur_token_is(&mut self, token_type: TokenTypes) -> bool {
        return self.cur_token.tokentype == token_type;
    }

    fn peek_token_is(&mut self, token_type: &TokenTypes) -> bool {
        return self.peek_token.tokentype == *token_type;
    }

    fn peek_error(&mut self, token_type: TokenTypes) {
        self.errors.push(ParseError::new(ParseErrorType::UnexpectedToken, format!(
                "expected next token to be {:?}, got {:?} instead",
                token_type, self.peek_token
            )));
    }

}

 
#[cfg(test)]
mod test {
    use super::*;

    fn check_parse_errors(parser: &mut Parser) {
        let errors = parser.errors.clone();

        if errors.len() == 0 {
            return;
        }

        println!("\n");

        println!("parser has {} errors", errors.len());

        for err in errors {
            println!("parse error: {:?}", err);
        }

        println!("\n");

        panic!("failed");
    }

    #[test]
    fn test_parser_let() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&mut parser);

        assert_eq!(program.len(), 3);

        assert_eq!(vec![
                Statement::Let(Ident(String::from("x")), Expression::Ident(Ident(String::from("")))),
                Statement::Let(Ident(String::from("y")), Expression::Ident(Ident(String::from("")))),
                Statement::Let(
                    Ident(String::from("foobar")),
                    Expression::Ident(Ident(String::from("")))
                ),
            ], program);
    }
    
    #[test]
    fn test_parser_return() {
        let input = "
            return 5;
            return 10;
            return 838383;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&mut parser);

        assert_eq!(program.len(), 3);

        assert_eq!(vec![
                Statement::Return(Expression::Ident(Ident(String::from("")))),
                Statement::Return(Expression::Ident(Ident(String::from("")))),
                Statement::Return(Expression::Ident(Ident(String::from("")))
                ),
            ], program);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&mut parser);

        assert_eq!(program.len(), 1);

        assert_eq!(vec![
                Statement::Expression(Expression::Ident(Ident(String::from("foobar")))),
            ], program);
    }

    #[test]
    fn test_integer_literal() {
        let input = "5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&mut parser);

        assert_eq!(program.len(), 1);

        assert_eq!(vec![
                Statement::Expression(Expression::IntegerLiteral(5)),
            ], program);
    }

    #[test]
    fn test_parse_prefix_expression() {
        let tests = vec![
            (
                "!5;",
                Statement::Expression(Expression::Prefix(Prefix::Not, Box::new(Expression::IntegerLiteral(5)))),
            ),
            (
                "-15;",
                Statement::Expression(Expression::Prefix(Prefix::Minus, Box::new(Expression::IntegerLiteral(15)))),
            )
        ];

        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse_program();

            check_parse_errors(&mut parser);
            assert_eq!(vec![expect], program);
        }
    }
}
