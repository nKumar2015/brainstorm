#[derive(Clone,Debug)] 
pub enum Program {
    Body{statements: Vec<Statement>},
}

#[derive(Clone,Debug)] 
pub enum Statement {
    Expression{expression: Expression},
    Assignment{name: String, rhs: Expression},
    OperatorAssignment{name: String, 
                       operator: Operator, 
                       rhs: Expression},
    
    If{condition: Expression, 
       statements: Vec<Statement>, 
       else_statements: Option<Vec<Statement>>},
    
    While{condition: Expression, statements: Vec<Statement>},

    For{control_var: String, initial: Expression, condition: Expression, 
        iterate_var: String, operator: Operator, iterate_exp: Expression, 
        statements: Vec<Statement>}
}

#[derive(Clone,Debug)] 
pub enum Expression {
    Int{v: i64},
    StringLiteral{s: String},
    Boolean{b: bool},
    Character{c: char},
    Identifier{name: String},
    Call{function: String, args: Vec<Expression>},
    Comparison{lhs: Box<Expression>, rhs: Box<Expression>, operator: Operator}
}

#[derive(Clone,Debug)] 
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    LessThan,
    GreaterThan,
    Equal
}


