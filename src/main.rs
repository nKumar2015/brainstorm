use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

mod ast;

use ast::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);


fn main() {
    let test = read_test();

    let mut enviornment = HashMap::new();
    enviornment.insert("print".to_string(), Value::Function{f: print_});

    let ast = parser::ProgramParser::new().parse(&test).unwrap();

    let result = eval_progam(&mut enviornment, ast);

    println!("{:?}", result);
}

fn read_test() -> String {
    let f = File::open("src/test.txt").unwrap();
    let mut lines = BufReader::new(f).lines();
    let mut test = String::new();

    loop{
        if let Some(s) = lines.next() {
            test.push_str(&s.unwrap());
        }else {
            break;
        }
    }

    return test;
}

fn eval_progam(enviornment: &mut HashMap<String, Value>, Program::Body{statements}: Program) -> Result<(), String>{
    for statement in statements {
        if let Err(e) = eval_statement(enviornment, statement) {
            return Err(e);
        }
    }

    Ok(())
}

fn eval_statement(enviornment: &mut HashMap<String, Value>, statement: Statement) -> Result<(), String> {
    match statement {
        Statement::Expression{expression} => {
            if let Err(e) = eval_expression(enviornment, expression) {
                return Err(e);
            }
        },
        Statement::Assignment{name, rhs} => {
            match eval_expression(enviornment, rhs) {
                Ok(v) => {
                    enviornment.insert(name, v);
                },
                Err(e) => return Err(e),
            }
        }
//        _ => return Err(format!("unhandled statement: {:?}", statement)),
    }

    Ok(())
}

fn eval_expression(enviornment: &mut HashMap<String, Value>, expression: Expression) -> Result<Value, String>{
    match expression {
        Expression::Int{v} => Ok(Value::Int{v}),
        Expression::Identifier{name} => {
            match enviornment.get(&name) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("'{}' is not defined", &name))
            }
        },
        Expression::Call{function, args} =>  {
            let mut vals = vec![];
            for arg in args {
                match eval_expression(enviornment, *arg) {
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e)
                }
            }
            
            let v = 
                match enviornment.get(&function) {
                    Some(v) => v,
                    None => return Err(format!("'{}' is not defined", &function))
                };
            
            if let Value::Function{f} = v {
                f(vals)
            }else{
                Err(format!("'{}' is not a function", function))
            }
        },
        Expression::Operation{lhs, rhs} => {
            let expressions = vec![lhs, rhs];
            let mut vals = vec![];

            for expression in expressions {
                match eval_expression(enviornment, *expression){
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e),
                }
            }

            if let [lhs, rhs] = vals.as_slice() {
                match (lhs, rhs){
                    (Value::Int{v: lhs}, Value::Int{v: rhs}) => {
                        Ok(Value::Bool{b: lhs < rhs})
                    },
                    _ => Err(format!("unhandled typ es: {:?}", (lhs, rhs))),
                }
            }else {
                Err(format!("dev error: "))
            }
        }
//        _ => Err(format!("unhandled expression: {:?}", expression)),
    }
}

#[derive(Clone,Debug)]
enum Value {
    Null,
    Int{v: i64},
    Bool{b: bool},
    Function{f: fn(Vec<Value>) -> Result<Value, String>},
}

fn print_(values: Vec<Value>) -> Result<Value, String> {
    println!("{values:?}");
    Ok(Value::Null)
}