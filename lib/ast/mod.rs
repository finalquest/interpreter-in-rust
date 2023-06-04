use crate::lexer::token::*;

pub type Program = Vec<Statement>;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    LetStatement {
        token: Token,
        name: Ident,
        value: Expression,
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Ident {

}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {

}
