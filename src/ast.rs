#[derive(Clone,Debug)] 
pub enum Program {
    Body{statements: Vec<Statement>},
}

#[derive(Clone,Debug)] 
pub enum Statement {
    Import{path: String},
    Expression{expression: Expression},
    Assignment{lhs: Expression, rhs: Expression},
    OperatorAssignment{name: String, 
                       operator: Operator, 
                       rhs: Expression},
    
    If{params: IfBranch},
    
    While{condition: Expression, statements: Vec<Statement>},
    
    For{params: ForLoop},

    FunctionDefinition{name: String, 
                       arguments: Vec<String>, 
                       statements: Vec<Statement>,
                       return_expression: Option<Expression> }
}

#[derive(Clone,Debug)] 
pub enum Expression {
    // BEGIN TYPES
    Int{v: i32},
    String{s: String},
    Boolean{b: bool},
    Float{f: f64},
    Character{c: char},
    List{items: Vec<ListItem>},
    // END TYPES

    Identifier{name: String},
    Call{function: String, arguments: Vec<Expression>},

    Operation{lhs: Box<Expression>, rhs: Box<Expression>, operator: Operator},
    Prefix{name: String, operator:Operator, rhs: Box<Expression>},

    Index{name: String, idx_exp: Box<Expression>}
}
#[derive(Clone,Debug)] 
pub struct ForLoop {
    pub loop_var: String,
    pub iterate_expression: Expression,
    pub statements: Vec<Statement>,
}

#[derive(Clone,Debug)] 
pub struct IfBranch {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub else_statements: Option<Vec<Statement>>
}

#[derive(Clone,Debug)] 
pub struct ListItem {
    pub expression: Expression,
    pub is_spread: bool,
    pub is_pack: bool,
}

#[derive(Clone,Debug)] 
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual
}
