use std::collections::HashMap;

use crate::ast::{Expression, Program, Statement, Operator};

pub fn eval_program(enviornment: &mut HashMap<String, Value>, 
    Program::Body{statements}: Program) -> Result<(), String> {
        
        eval_block(enviornment, statements)
}

fn eval_block(enviornment: &mut HashMap<String, Value>, 
              statements: Vec<Statement>) -> Result<(), String> {
    
    for statement in statements {
        eval_statement(enviornment, statement)?;
    }

    Ok(())
}

fn eval_statement(enviornment: &mut HashMap<String, Value>, 
    statement: Statement) -> Result<(), String> {
    match statement {
        Statement::Expression{expression} => {
            eval_expression(enviornment, expression)?;
        },
        Statement::Assignment{name, rhs} => {
            match eval_expression(enviornment, rhs) {
                Ok(v) => {
                    enviornment.insert(name, v);
                },
                Err(e) => return Err(e),
            }
        },
        Statement::OperatorAssignment{name, operator, rhs} => {
            let lhs = 
                match enviornment.get(&name) {
                    Some(v) => v.clone(),
                    None => return Err(format!("'{}' is not defined", &name))
                };

            let rhs = match eval_expression(enviornment, rhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

            let v = 
                match operate(operator.expect("msg"), &lhs, &rhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

            enviornment.insert(name, v);
        },
        Statement::If{condition, statements,
                      else_statements } => {
            match eval_expression(enviornment, condition){
                Ok(Value::Bool { b: true }) => {
                    eval_block(enviornment, statements)?;
                },
                Ok(Value::Bool { b: false }) => {
                    if let Some(statements) = else_statements {
                        eval_block(enviornment, statements)?;
                    }
                },
                _ => return Err("Condition must be of type 'bool'".to_string()),
            }
        },
        //_ => return Err(format!("unhandled statement: {:?}", statement)),
    }

    Ok(())
}

fn eval_expression(enviornment: &mut HashMap<String, Value>, 
    expression: Expression) -> Result<Value, String>{
    match expression {
        Expression::Int{v} => Ok(Value::Int{v}),
        Expression::StringLiteral{ s } => Ok(Value::Str{s}),
        Expression::Identifier{name} => {
            match enviornment.get(&name) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("'{}' is not defined", &name))
            }
        },
        Expression::Call{function, args} =>  {
            let mut vals = vec![];
            for arg in args {
                match eval_expression(enviornment, arg) {
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e)
                }
            }
            
            let Some(v) = enviornment.get(&function) 
                else { return Err(format!("'{}' is not defined", &function)) };
            
            if let Value::Function{f} = v {
                f(vals)
            }else{
                Err(format!("'{function}' is not a function"))
            }
        },
        Expression::Comparison{lhs, 
                               rhs, 
                               operator} => {
            let expressions = vec![lhs, rhs];
            let mut vals = vec![];

            for expression in expressions {
                match eval_expression(enviornment, *expression){
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e),
                }
            }

            if let [lhs, rhs] = vals.as_slice() {
                    operate(operator, lhs, rhs)
            }else {
                Err("dev error: ".to_string())
            }
        }
      //_ => Err(format!("unhandled expression: {:?}", expression)),
    }
}

fn operate(operator: Operator, lhs: &Value, rhs: &Value) 
    -> Result<Value, String>{
    match (lhs, rhs){
        (Value::Int{v: lhs}, Value::Int{v: rhs}) => {
            match operator {
                Operator::Plus => {Ok(Value::Int { v: lhs + rhs })},
                Operator::Minus => {Ok(Value::Int { v: lhs - rhs })},
                Operator::Times => {Ok(Value::Int { v: lhs * rhs })},
                Operator::Divide => {Ok(Value::Int { v: lhs / rhs })},
                Operator::LessThan => {Ok(Value::Bool { b: lhs < rhs })},
                Operator::GreaterThan => {Ok(Value::Bool { b: lhs > rhs })},
                Operator::Equal => {Ok(Value::Bool { b: lhs == rhs })},
            } 
        },
        _ => Err(format!("unhandled types: {:?}", (lhs, rhs))),
    }
}               

#[derive(Clone,Debug)]
pub enum Value {
    Null,
    Int{v: i64},
    #[allow(dead_code)]
    Str{s: String},
    Bool{b: bool},
    Function{f: fn(Vec<Value>) -> Result<Value, String>},
}