#[derive(Clone,Debug)] 
pub enum Program {
    Body{statements: Vec<Statement>},
}

#[derive(Clone,Debug)] 
pub enum Statement {
    Expression{expression: Expression},
    Assignment{name: String, rhs: Expression},
}

#[derive(Clone,Debug)] 
pub enum Expression {
    Int{v: i64},
    Identifier{name: String},
    Call{function: String, args: Vec<Box<Expression>>},
    Operation{lhs: Box<Expression>, rhs: Box<Expression>}
}