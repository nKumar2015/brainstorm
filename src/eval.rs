use std::collections::HashMap;

use crate::ast::{Expression, Operator, Program, Statement};
use crate::constants::FP_ERROR_MARGIN;

pub fn eval_program(enviornment: &mut HashMap<String, Value>, 
    Program::Body{statements}: &Program) -> Result<(), String> {
        
        eval_statements(enviornment, statements)
}

fn eval_statement(enviornment: &mut HashMap<String, Value>, 
    statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::Expression{expression} => {
            eval_expression(enviornment, expression)?;
        },
        Statement::Assignment{name, rhs} => {
            match eval_expression(enviornment, rhs) {
                Ok(v) => {
                    enviornment.insert(name.clone(), v);
                },
                Err(e) => return Err(e),
            }
        },
        Statement::OperatorAssignment{name, operator, rhs} => {
            let lhs = 
                match enviornment.get(name) {
                    Some(v) => v.clone(),
                    None => return Err(format!("'{}' is not defined", &name))
                };

            let rhs = match eval_expression(enviornment, rhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

            let v = 
                match operate(operator, &lhs, &rhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

            enviornment.insert(name.clone(), v);
        },
        Statement::If{params} => {
            match eval_expression(enviornment, &params.condition) {
                Ok(Value::Bool{b: true}) 
                    => eval_statements(enviornment, &params.statements)?,
                Ok(Value::Bool{b: false}) 
                    => {
                        if let Some(else_statements) 
                            = &params.else_statements { 
                            eval_statements(enviornment, else_statements)?;
                        }
                    },
                _ => return Err("Condition must be of type 'bool'".to_string()),
            }
        },
        Statement::While{condition, statements} => {            
            loop{
                let b = 
                    match eval_expression(enviornment, condition) {
                        Ok(Value::Bool{b}) => b ,
                        Err(e) => return Err(e),
                        _ => return Err(
                            "Condition must be of type 'bool'".to_string()),
                    };
                            
                if !b { break; }
                
                #[allow(clippy::question_mark)]
                if let Err(e) = eval_statements(enviornment, statements) {
                    return Err(e);
                }
            }
        },
        Statement::For{params} => {

            match eval_statement(enviornment, &params.initialization_statment) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }

            loop {
                let b = 
                    match eval_expression(enviornment, 
                            &params.iteration_condition) {
                        Ok(Value::Bool{b}) => b,
                        Err(e) => return Err(e),
                        _ => return Err(
                            "Condition must evaluate to a bool".to_string())
                    };

                if !b { break; }
                
                eval_statements(enviornment, &params.statements)?;

                eval_statement(enviornment, 
                              &params.iteration_variable_statement)?;
            }
        },

        //_ => return Err(format!("unhandled statement: {:?}", statement)),
    }

    Ok(())
}

fn eval_statements(enviornment: &mut HashMap<String, Value>, 
              statements: &Vec<Statement>) -> Result<(), String> {
    
    for statement in statements {
        eval_statement(enviornment, statement)?;
    }

    Ok(())
}

fn eval_expression(enviornment: &mut HashMap<String, Value>, 
    expression: &Expression) -> Result<Value, String>{
    match expression {
        Expression::Int{v} => Ok(Value::Int{v: *v}),
        Expression::StringLiteral{ s } => Ok(Value::Str{s: s.clone()}),
        Expression::Boolean{ b } => Ok(Value::Bool{b: *b}),
        Expression::Float{ f} => Ok(Value::Float{f: *f}),
        Expression::Character{ c } => Ok(Value::Char{c: *c}),
        Expression::Identifier{name} => {
            match enviornment.get(name) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("'{}' is not defined", &name))
            }
        },
        Expression::Call{function, args} =>  {
            let vals = eval_expressions(enviornment, args)?;
            
            let Some(v) = enviornment.get(function) 
                else { return Err(format!("'{}' is not defined", &function)) };
            
            if let Value::Function{f} = v {
                f(vals)
            }else{
                Err(format!("'{function}' is not a function"))
            }
        },
        Expression::Operation { lhs, rhs, operator } => {
            let expressions = vec![lhs, rhs];
            let mut vals = vec![];

            for expression in expressions {
                match eval_expression(enviornment, expression) {
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e),
                }
            }

            if let [lhs, rhs] = vals.as_slice() {
                operate(operator, lhs, rhs)
            }else{
                Err("dev error: ".to_string())
            }
        },
        Expression::List { items} => {
            let mut vals: Vec<Value> = vec![];
            
            for item in items {
                let v = 
                    match eval_expression(enviornment, &item.expression) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    };

                if !item.is_spread {
                    vals.push(v);
                    continue;
                }

                match v {
                    Value::List{mut e} => vals.append(&mut e),
                    _ => return Err("only lists can be spread!".to_string())
                }
            }

            Ok(Value::List{e: vals})
        },

        //_ => Err(format!("unhandled expression: {:?}", expression)),
    }
}

fn eval_expressions(enviornment: &mut HashMap<String, Value>, 
    expressions: &Vec<Expression>) -> Result<Vec<Value>, String> {
        let mut vals = vec![];

        for expression in expressions {
            match eval_expression(enviornment, expression) {
                Ok(v) => vals.push(v),
                Err(e) => return Err(e),
            }
        }

        Ok(vals)
}

fn operate(operator: &Operator, lhs: &Value, rhs: &Value) 
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
                Operator::NotEqual => {Ok(Value::Bool { b: lhs != rhs })}
            } 
        },
        (Value::Float{f: lhs}, Value::Float{f: rhs}) => {
            match operator{
                Operator::Plus => {Ok(Value::Float { f: lhs + rhs })},
                Operator::Minus => {Ok(Value::Float { f: lhs - rhs })},
                Operator::Times => {Ok(Value::Float { f: lhs * rhs })},
                Operator::Divide => {Ok(Value::Float { f: lhs / rhs })},
                Operator::LessThan => {Ok(Value::Bool { b: lhs < rhs })},
                Operator::GreaterThan => {Ok(Value::Bool { b: lhs > rhs })},
                Operator::Equal => {Ok(Value::Bool { 
                    b: (lhs - rhs).abs() < FP_ERROR_MARGIN 
                })},
                Operator::NotEqual => {Ok(Value::Bool { 
                    b: (lhs - rhs).abs() > FP_ERROR_MARGIN 
                })},
            }
        },
        (Value::Float{f: lhs}, Value::Int{v: rhs}) => {
            let rhsf = f64::from(*rhs);
            match operator{
                Operator::Plus => {Ok(Value::Float { f: lhs + rhsf })},
                Operator::Minus => {Ok(Value::Float { f: lhs - rhsf })},
                Operator::Times => {Ok(Value::Float { f: lhs * rhsf })},
                Operator::Divide => {Ok(Value::Float { f: lhs / rhsf })},
                Operator::LessThan => {Ok(Value::Bool { b: *lhs < rhsf })},
                Operator::GreaterThan => {Ok(Value::Bool { b: *lhs > rhsf })},
                Operator::Equal => {Ok(Value::Bool { 
                    b: (*lhs - rhsf).abs() < FP_ERROR_MARGIN 
                })},
                Operator::NotEqual => {Ok(Value::Bool { 
                    b: (*lhs - rhsf).abs() > FP_ERROR_MARGIN 
                })},

            }
        },
        (Value::Int{v: lhs}, Value::Float{f: rhs}) => {
            let lhsf = f64::from(*lhs);
            match operator{
                Operator::Plus => {Ok(Value::Float { f: lhsf + rhs })},
                Operator::Minus => {Ok(Value::Float { f: lhsf - rhs })},
                Operator::Times => {Ok(Value::Float { f: lhsf * rhs })},
                Operator::Divide => {Ok(Value::Float { f: lhsf / rhs })},
                Operator::LessThan => {Ok(Value::Bool { b: lhsf < *rhs })},
                Operator::GreaterThan => {Ok(Value::Bool { b: lhsf > *rhs })},
                Operator::Equal => {Ok(Value::Bool { 
                    b: (lhsf - *rhs).abs() < FP_ERROR_MARGIN 
                })},
                Operator::NotEqual => {Ok(Value::Bool { 
                    b: (lhsf - *rhs).abs() > FP_ERROR_MARGIN 
                })},
            }
        }
        _ => Err(format!("unhandled types: {:?}", (lhs, rhs))),
    }
}               

#[derive(Clone,Debug)]
pub enum Value {
    Null,
    Int{v: i32},
    #[allow(dead_code)]
    Str{s: String},
    Bool{b: bool},
    #[allow(dead_code)]
    Float{f: f64},
    #[allow(dead_code)]
    Char{c: char},
    #[allow(dead_code)]
    List{e: Vec<Value>},
    Function{f: fn(Vec<Value>) -> Result<Value, String>},
}