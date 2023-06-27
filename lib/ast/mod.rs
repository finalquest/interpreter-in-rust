pub type Program = Vec<Statement>;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Ident,Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Not,
    Minus,
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Ident(Ident),
    IntegerLiteral(i64),
    Prefix(Prefix, Box<Expression>),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(x)
    Index,       // array[index]
}
