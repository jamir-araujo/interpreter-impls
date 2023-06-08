pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Let),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(String),
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub name: String,
    pub value: Expression,
}

pub struct Identifier {
    pub value: String,
}
