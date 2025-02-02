use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

mod ast; 
mod eval;
mod constants;

use eval::Value;

#[macro_use]
extern crate lalrpop_util; 

lalrpop_mod!(pub parser);


fn main() {
    let test = read_test();

    let mut enviornment = HashMap::new();
    enviornment.insert("print".to_string(), 
        eval::Value::Function{f: print_});

    println!("AST OUTPUT: \n");

    let ast = parser::ProgramParser::new().parse(&test).unwrap();
    println!("{ast:?}\n");

    println!("PROGRAM OUTPUT: \n");

    let result = eval::eval_program(&mut enviornment, &ast);

    println!("{result:?}");
}

fn read_test() -> String {
    let f = File::open("src/test.txt").unwrap();
    let lines = BufReader::new(f).lines();
    let mut test = String::new();

    for s in lines{
        test.push_str(&s.unwrap());
    }

    test
}

#[allow(clippy::unnecessary_wraps)]
fn print_(values: Vec<Value>) -> Result<Value, String> {
    println!("{values:?}");
    Ok(Value::Null)
}