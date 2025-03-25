use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

mod ast; 
mod eval;
mod constants;
mod value;

use crate::value::Value;

#[macro_use]
extern crate lalrpop_util; 

lalrpop_mod!(pub parser);


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let file = match read_file(&args[1]) {
        Ok(file) => file,
        Err(e) => panic!("{}", e)
    };

    let mut enviornment = HashMap::new();
    enviornment.insert("println".to_string(), 
        Value::Function{name: "println".to_string(), f: println_});
    
    enviornment.insert("print".to_string(), 
        Value::Function{name: "print".to_string(), f: print_});

    enviornment.insert("range".to_string(), 
        Value::Function{name: "range".to_string(), f: range});

    enviornment.insert("range_step".to_string(), 
        Value::Function{name: "range_step".to_string(), f: range_step});
        
    let ast = parser::ProgramParser::new().parse(&file).unwrap();
    
    let result = eval::eval_program(&mut enviornment, &ast, false);

    match &result {
        Ok(()) => (),
        Err(s) => {
            println!("\nError: {s}\n\nAST:\n{:?}\n", ast);
        },
    }
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let lines = BufReader::new(f).lines();
    let mut file = String::new();

    for s in lines{
        match s {
            Ok(s) => file.push_str(&s),
            Err(e) => return Err(e)
        }
    }

    Ok(file)
}



#[allow(clippy::unnecessary_wraps)]
pub fn println_(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        print!("{}", arg);
    }
    println!();
    Ok(Value::Null)
}

pub fn print_(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        print!("{}", arg);
    }
    Ok(Value::Null)
}

#[allow(clippy::missing_panics_doc)]
pub fn range_step(args: Vec<Value>) -> Result<Value, String> {
    let mut vals = vec![];

    let start = args[0].clone();
    let end = args[1].clone();
    let step = args[2].clone();

    let Value::Int{v: s} = start 
        else { return Err("Invalid Type".to_string())};
    
    let Value::Int{v: e} = end 
        else { return Err("Invalid Type".to_string())};

    let Value::Int{v: st} = step 
        else { return Err("Invalid Type".to_string())};
    
    for x in (s..e).step_by(st.try_into().unwrap()) {
        vals.push(Value::Int{v: x});
    }

    Ok(Value::List{e: vals})
}

pub fn range(args: Vec<Value>) -> Result<Value, String> {
    let mut vals = args;
    vals.push(Value::Int{v: 1});
    range_step(vals)
}


