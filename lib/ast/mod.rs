pub type Program = Vec<Statement>;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Ident,Expression),
    Return(Expression)
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Ident(Ident),
}
