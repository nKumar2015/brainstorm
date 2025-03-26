use std::collections::HashMap;
use std::env::{ args, current_dir, var};
use std::path::Path;

use crate::ast::{ClassField, Expression, IfBranch, ListItem, 
                 Operator, Program, Statement};
use crate::parser::ProgramParser;
use crate::read_file;
use crate::value::Value;
use crate::constants::KEYWORDS;
use crate::{println_, print_, range_step, range};

pub fn eval_program(enviornment: &mut HashMap<String, Value>, 
                    Program::Body{statements}: &Program, importing: bool) 
                    -> Result<(), String> {
        
        eval_statements(enviornment, statements, importing)
}

fn assign(enviornment: &mut HashMap<String, Value>, lhs: Expression, rhs: Value)
    -> Result<(), String> {

    match lhs {
        Expression::Identifier { name } => {
                    if name == "_" {
                        return Ok(());
                    }
                    enviornment.insert(name.clone(), rhs);
        },
        Expression::List { items } => {
            let Value::List{e: new_items} = rhs 
            else { 
                return Err("cannot destructure non-list into list".to_string()) 
            };

            assign_list(enviornment, items, new_items)?;
        },
        Expression::Index { name, idx_exp} => {
            let Some(var) = enviornment.get(&name) 
                else { return Err(format!("'{}' is not defined", name)) };
            

            let exp_res = 
                match eval_expression(&mut enviornment.clone(), 
                          &idx_exp, false){
                    Ok(v) => v,
                    Err(e) => return Err(e),
            };

            let mut list = match var {
                Value::List { e } => e.clone(),

                Value::Str { .. } 
                    => return Err("Cannot assign to String Index".to_string()),
                Value::Null 
                    => return Err("Cannot index Null".to_string()),
                Value::Int { .. } 
                    => return Err("Cannot index Int".to_string()),
                Value::Bool { .. } 
                    => return Err("Cannot index Boolean".to_string()),
                Value::Char { .. } 
                    => return Err("Cannot index Char".to_string()),
                Value::Function { .. } 
                    => return Err("Cannot index Function".to_string()),
                Value::UserDefFunction { .. } 
                    => return Err("Cannot index Function".to_string()),
                Value::Float { .. } 
                    => return Err("Cannot index Float".to_string()),
                Value::Object { .. }
                    => return Err("Cannot index Object".to_string()),
            };

            let Value::Int { v: idx } = exp_res 
                else { return Err("Index must be of type int".to_string()) };

            let usize_idx = idx.unsigned_abs() as usize;
            let length = list.len();
            if usize_idx > length {
                return Err(format!("Index {} is out of bounds", idx));
            }

            if idx < 0 {
                list[length - usize_idx] = rhs;
            }else{
                list[usize_idx] = rhs;
            }
            

            enviornment.insert(name, Value::List { e: list });
        },
        Expression::FieldAccess { name, field } => {
            if name == "this" {
                let Some(_) = enviornment.get(&field) else {
                    return Err(format!("No such field \"{}\"", field))
                };
                enviornment.insert(field, rhs);
                return Ok(())
            }

            let Some(obj) = enviornment.get(&name) else {
                return Err(format!("{} is undefined", name))
            };

            let Value::Object { name: class_name, 
                                fields: obj_fields, 
                                init, methods, 
                                parent_class } 
                                = obj else { 
                return Err(format!("{} is not an object", name)) 
            };

            let Some(data) = obj_fields.get(&field) else {
                return Err(format!("{} has no field {}", name, field))
            };

            if data.is_private {
                return Err("Cannot access private field".to_string())
            }

            let new_field = ClassField{
                is_private: data.is_private,
                value: rhs,
            };

            let mut new_fields = obj_fields.clone();
            new_fields.insert(field, new_field);

            let updated_obj = Value::Object { 
                name: class_name.to_string(), fields: new_fields, 
                init: init.clone(), methods: methods.clone(), 
                parent_class: parent_class.clone()
            };

            enviornment.insert(name, updated_obj);

            return Ok(())
        }, 
        Expression::Int { .. } 
            => return Err("Cannot assign to a Integer literal".to_string()),
        Expression::String { .. } 
            => return Err("Cannot assign to a String literal".to_string()),
        Expression::Boolean { ..} 
            => return Err("Cannot assign to a Boolean literal".to_string()),
        Expression::Float { .. } 
            => return Err("Cannot assign to a Float literal".to_string()),
        Expression::Character { .. } 
            => return Err("Cannot assign to a Character literal".to_string()),
        Expression::Call { ..} 
            => return Err("Cannot assign to a Function call".to_string()),
        Expression::Operation { .. } 
            => return Err("Cannot assign to a Operation".to_string()),
        Expression::Prefix { .. } 
            => return Err("Cannot assign to a Prefix".to_string()),
        Expression::Comprehension { .. } 
            => return Err("Cannot assign to a Comprehension".to_string()),
        Expression::ClassDef { .. }
            => return Err("Cannot assign to a Class".to_string()),
        Expression::ObjectCreation { .. } 
            => return Err("Cannot assign to a Object".to_string()),
        Expression::MethodCall { .. } => {
            todo!();
        }
    }



    Ok(())
}

fn assign_list(enviornment: &mut HashMap<String, Value>, lhs: Vec<ListItem>, 
    rhs: Vec<Value>) -> Result<(), String> {

    if lhs.len() > rhs.len() {
        return Err(format!("Cannot assign {} values to {} items", 
                    rhs.len(), 
                    lhs.len()))
    }

    let mut assign_name_queue: Vec<ListItem> = vec![];
    let mut assign_value_queue: Vec<Value> = vec![];

    for x in 0..rhs.len(){
        if x == lhs.len() - 1 && lhs.len() != rhs.len(){
            if !lhs[x].is_pack {
                return Err(format!("Cannot assign {} values to {} items", 
                    rhs.len(), 
                    lhs.len()))
            }

            assign_name_queue.push(lhs[x].clone());
            assign_value_queue.push(Value::List{e: rhs[x..].to_vec()});
            break;
        }

        if lhs[x].is_spread {
            return Err("Cannot use spread in list assignment".to_string())
        }

        assign_name_queue.push(lhs[x].clone());
        assign_value_queue.push(rhs[x].clone());
    }

    for (ListItem{expression, .. }, value) in
        assign_name_queue.into_iter().zip(assign_value_queue.into_iter()) {
        
        assign(enviornment, expression, value)?;
    }

    Ok(())

}

fn eval_statement(enviornment: &mut HashMap<String, Value>, 
    statement: &Statement, importing: bool) -> Result<(), String> {
    match statement {
        Statement::Expression{expression} => {
            eval_expression(enviornment, expression, importing)?;
        },
        Statement::Assignment{lhs, rhs} => {
            let v = 
                match eval_expression(enviornment, rhs, importing) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
            
            assign(enviornment, lhs.clone(), v)?;
        },
        Statement::OperatorAssignment{name, operator, rhs} => {
            let lhs = 
                match enviornment.get(name) {
                    Some(v) => v.clone(),
                    None => return Err(format!("'{}' is not defined", &name))
                };

            let rhs = match eval_expression(enviornment, rhs, importing) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

            let v = 
                match operate(operator, &lhs, &rhs) {
                    Ok(Value::Null) 
                        => return Err(format!("Cannot operate on {}", name)),
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

            enviornment.insert(name.clone(), v);
        },
        Statement::If{params} => {
            match eval_expression(enviornment, &params.condition, importing) {
                Ok(Value::Bool{b: true}) 
                    => eval_statements(enviornment, &params.statements, 
                                       importing)?,
                Ok(Value::Bool{b: false}) => {
                    let (elif_conditions, elif_statements ) = &params.elif_data;
                    if !elif_conditions.is_empty() {
                        let condition = elif_conditions[0].clone();
                        let statement = elif_statements[0].clone();

                        let next_iter = IfBranch{
                            condition,
                            statements: statement,
                            else_statements: params.else_statements.clone(),
                            elif_data: (elif_conditions[1..].to_vec(), 
                                        elif_statements[1..].to_vec())
                        };

                        eval_statement(enviornment, 
                            &Statement::If{params: next_iter}, importing)?;
                    }else if let Some(else_statements) = 
                        &params.else_statements { 
                            eval_statements(enviornment, else_statements, 
                                            importing)?;
                    }
                },
                _ => return Err("Condition must be of type 'bool'".to_string()),
            }
        },
        Statement::While{condition, statements} => {            
            loop{
                let b = 
                    match eval_expression(enviornment, condition, importing) {
                        Ok(Value::Bool{b}) => b ,
                        Err(e) => return Err(e),
                        _ => return Err(
                            "Condition must be of type 'bool'".to_string()),
                    };
                            
                if !b { break; }
                
                if let Err(e) 
                    = eval_statements(enviornment, statements, importing) {
                    return Err(e);
                }
            }
        },
        Statement::For{params} => {
            let v = 
            match &params.iterate_expression {
                Expression::List { .. } 
                    => eval_expression(enviornment, 
                                      &params.iterate_expression, importing)?,
                Expression::Identifier { .. } 
                    => eval_expression(enviornment, 
                                      &params.iterate_expression, importing)?,
                Expression::Call { .. } 
                    => eval_expression(enviornment, 
                                      &params.iterate_expression, importing)?,
                Expression::Int { .. } 
                    => return Err(
                        "Integer literals are not iterable".to_string()),
                Expression::String { .. } 
                    => return Err(
                        "String literals are not iterable".to_string()),
                Expression::Boolean { .. } 
                    => return Err(
                        "Boolean literals are not iterable".to_string()),
                Expression::Float { .. } 
                    => return Err(
                        "Float literals are not iterable".to_string()),
                Expression::Character { .. } 
                    => return Err(
                        "Character literals are not iterable".to_string()),
                Expression::Operation { .. } 
                    => return Err(
                        "Operations are not iterable".to_string()),
                Expression::Prefix { .. } 
                    => return Err(
                        "Prefix's are not iterable".to_string()),
                Expression::Index { .. } 
                    => return Err(
                        "Indexes are not iterable".to_string()),
                Expression::Comprehension { .. } 
                    => return Err(
                        "Comprehensions are not iterable".to_string()),
                Expression::ClassDef { .. }
                    => return Err(
                        "Classes are not iterable".to_string()),
                Expression::FieldAccess { .. }
                    => return Err(
                        "Fields are not are not iterable".to_string()),
                Expression::ObjectCreation { .. }
                    => return Err(
                        "Objects are not are not iterable".to_string()),
                Expression::MethodCall { .. } => {
                    todo!();
                }
            };

            let Value::List{e: iterator_list} = v 
                else { return Err("Invalid Type".to_string())};

            for list_item in iterator_list {
                enviornment.insert(params.loop_var.clone(), list_item);

                eval_statements(enviornment, &params.statements, importing)?;
            }
        },
        Statement::FunctionDefinition { name, arguments, 
                                        statements, return_expression } => {
            if enviornment.get(name).is_some() {
                return Err("Function '{}' is already defined!".to_string());
            }

            enviornment.insert(name.to_string(), 
                               Value::UserDefFunction { 
                                    name: name.to_string(),
                                    statements: statements.clone(),
                                    arguments: arguments.clone(),
                                    return_expression: return_expression.clone()
                                });
        },
        Statement::Import{path} => {    
            // Get the provided path to file 
            // and the directory the executable was called from

            let args: Vec<String> = args().collect();
            let cwd = current_dir().unwrap();
            
            // The provided path
            let origin_file: &String = &args[1];

            // replace "." with the current working directory
            let mut full_path = origin_file.clone();
            if full_path.starts_with('.') {
                full_path = origin_file.replacen('.', 
                                    cwd.to_str().unwrap(),
                                    1);
            }
            let external_code = 
                if path.starts_with('.') {                    
                    // Move one level up
                    let parent_dir 
                        = Path::new(&full_path).parent().unwrap();

                    // replace the "." from the provided import path with the
                    // parent directory we found earlier
                    let full_import_path = 
                        path.replacen('.', 
                                    parent_dir.to_str().unwrap(), 
                                    1);
                    
                    // attempt to read that file
                    match read_file(&full_import_path) {
                        Ok(f) => f,
                        Err(_) => 
                            return Err(format!("Error opening file at {}", 
                                            full_import_path))
                    } 
                } else if path.contains('/'){
                    match read_file(path) {
                        Ok(f) => f,
                        Err(_) => return 
                            Err(format!("Error opening file at {}", path))
                    } 
                } else {
                    // Move one level up
                    let parent_dir 
                        = Path::new(&full_path).parent().unwrap();
                    let final_dir 
                        = format!("{}/{}", parent_dir.to_str().unwrap(), path); 
                    let result = read_file(&final_dir);

                    // If the file is present in the same directory, use that
                    #[allow(clippy::unnecessary_unwrap)]
                    if result.is_ok() {
                        result.unwrap()
                    }else {
                        // If the file is not present, check if the file exists 
                        // in the paths listedn in the BRNSTM_LIB env var 
                        let var = var("BRNSTM_LIB");
                        let mut out = String::new();
                        if var.is_ok(){
                            let res_val = var.unwrap();
                            let paths = res_val.split(':');
                            for dir in paths {
                                let lib_path = format!("{}/{}", dir, path);
                                let res = read_file(&lib_path);

                                if res.is_ok() {
                                    out = res.unwrap();
                                    break;
                                }
                            }
                        }
                        if out.is_empty() {
                            return Err(format!("Error opening file at {}", 
                                       path));
                        }
                        out.to_string()
                    }
                };
            let ast = ProgramParser::new().parse(&external_code).unwrap();

            eval_program(enviornment, &ast, true)?;
        },
    }

    Ok(())
}

fn eval_statements(enviornment: &mut HashMap<String, Value>, 
                   statements: &Vec<Statement>, 
                   importing: bool) -> Result<(), String> {
    
    for statement in statements {
        eval_statement(enviornment, statement, importing)?;
    }

    Ok(())
}

fn eval_expression(enviornment: &mut HashMap<String, Value>, 
    expression: &Expression, importing: bool) -> Result<Value, String>{
    match expression {
        Expression::Int{v} => Ok(Value::Int{v: *v}),
        Expression::String{ s } => Ok(Value::Str{s: s.clone()}),
        Expression::Boolean{ b } => Ok(Value::Bool{b: *b}),
        Expression::Float{ f} => Ok(Value::Float{f: *f}),
        Expression::Character{ c } => Ok(Value::Char{c: *c}),
        Expression::Identifier{name} => {
            match enviornment.get(name) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("'{}' is not defined", &name))
            }
        },
        Expression::Call{function, arguments} =>  {
            let vals = eval_expressions(enviornment, arguments, importing)?;

            let Some(v) = enviornment.get(function) 
                else { return Err(format!("'{}' is not defined", &function)) };
            
            let mut local_env = enviornment.clone();

            match v {
                Value::Function{f, ..} => {
                    if importing && (function == "print" || 
                                     function == "println" ) {

                            return Ok(Value::Null);     
                    }
                    f(vals)
                },
                Value::UserDefFunction {statements, 
                                        arguments , return_expression, ..} => {
                    if vals.len() != arguments.len() {
                        return Err(format!("Expected {} arguments, got {}", 
                                            arguments.len(), 
                                            vals.len()))
                    }
                    for (value, name) in vals.iter().zip(arguments.iter()) {
                        local_env.insert(name.to_string(), value.clone());
                    }
                    eval_statements(&mut local_env, statements, importing)?;
                    
                    match return_expression {
                        Some(return_exp) => {
                            match eval_expression(&mut enviornment.clone(),
                                      return_exp, importing) {
                                Ok(v) => Ok(v.clone()),
                                Err(e) 
                                    => Err(e)
                            }
                        },
                        None => Ok(Value::Null)
                    }

                },
                Value::Object{ .. } => {
                    if function != "super" {
                        return Err(format!("'{function}' is not a function"))
                    }

                    let exp = Expression::ObjectCreation{ 
                                            class_name: "super".to_string(), 
                                            arguments: arguments.clone()
                                        };

                    let result = eval_expression(enviornment, &exp, importing)?;
                    
                    match result {
                        Value::Object{fields, ..} => {
                            for (name, data) in fields {
                                let value = data.value;
                                enviornment.insert(name, value);
                            }
                        },
                        _ => return Err(
                            "Dev error non-object from parent initalization"
                            .to_string())
                    };


                    Ok(Value::Null)
                },
                _ => Err(format!("'{function}' is not a function"))
            }
        },
        Expression::Operation { lhs, rhs, operator } => {
            let expressions = vec![lhs, rhs];
            let mut vals = vec![];

            for expression in expressions {
                match eval_expression(enviornment, expression, importing) {
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e),
                }
            }

            if let [lhs, rhs] = vals.as_slice() {
                let new_val = operate(operator, lhs, rhs)?;
                if new_val == Value::Null {
                    return Err("Invalid Operation".to_string())
                }
                Ok(new_val)
            }else{
                Err("dev error: ".to_string())
            }
        },
        Expression::List { items} => {
            let mut vals: Vec<Value> = vec![];
            
            for item in items {
                let v = 
                    match eval_expression(enviornment, 
                                          &item.expression, 
                                          importing) {
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
        Expression::Prefix { name, operator, rhs } => {
            let lhs = match enviornment.get(name) {
                Some(v) => v.clone(),
                None => return Err(format!("'{}' is not defined", name))
            };

            let v = match eval_expression(enviornment, rhs, importing) {
                Ok(v) => v,
                Err(e) => return Err(e)
            };

            let new_val = operate(operator, &lhs, &v)?;
            if new_val == Value::Null {
                return Err(format!("Cannot operate on {}", name))
            }
            enviornment.insert(name.clone(), new_val.clone());

            Ok(new_val)
        },
        Expression::Index { name, idx_exp } => {
            let Some(var) = enviornment.get(name) 
                else { return Err(format!("'{}' is not defined", name)) };

            let exp_res = eval_expression(&mut enviornment.clone(), idx_exp, 
                                                 importing)?;

            let Value::Int { v: idx } = exp_res 
                else { return Err("Index must be of type int".to_string()) };

            let mut iterator = var.clone().into_iter();
            let length = iterator.clone().count();

            if iterator.value == Value::Null {
                return Err(format!("Cannot iterate over variable {}", name))
            }

            let usize_idx = idx.unsigned_abs() as usize;

            if usize_idx >= length {
                return Err(format!("Index {} is out of bounds", idx))
            }

            if idx < 0 {
                return Ok(iterator.nth(length - usize_idx)
                    .unwrap_or_else(|| panic!("Err retreiving value at {}", 
                                               idx)))
            }

            Ok(iterator.nth(usize_idx)
                .unwrap_or_else(|| panic!("Err retreiving value at {}", idx)))
        },
        Expression::Comprehension { iterate_exp, var, control_exp } => {
            let mut local_env = enviornment.clone();
            let control_val = eval_expression(&mut local_env, 
                                                      control_exp, importing)?;

            match control_val {
                Value::List { e } => {
                    let mut output = vec![];
                    for item in e {
                        local_env.insert(var.to_string(), item);
                        let iterate_exp_val = 
                            eval_expression(&mut local_env, 
                                             iterate_exp, importing)?;
                        output.push(iterate_exp_val);
                    }
                    Ok(Value::List{e: output})
                },
                Value::Str { s } => {
                    let mut output = vec![];
                    for c in s.chars() {
                        local_env.insert(var.to_string(), Value::Char {c});
                        let iterate_exp_val = 
                            eval_expression(&mut local_env, 
                                             iterate_exp, importing)?;

                        output.push(iterate_exp_val);
                    }
                    Ok(Value::List{e: output})
                },
                Value::Null 
                    => Err("Null is not iterable".to_string()),
                Value::Int { .. } 
                    => Err("Int is not iterable".to_string()),
                Value::Bool { .. } 
                    => Err("Bool is not iterable".to_string()),
                Value::Float { .. } 
                    => Err("Float is not iterable".to_string()),
                Value::Char { .. } 
                    => Err("Char is not iterable".to_string()),
                Value::Function { .. } 
                    => Err("Function is not iterable".to_string()),
                Value::UserDefFunction { .. } 
                    => Err("Function is not iterable".to_string()),
                Value::Object { .. }
                    => Err("Class is not iterable".to_string())
            }
        },
        Expression::ClassDef { params } => {
            let name = &params.name;
            let fields = &params.fields;
            
            for word in KEYWORDS {
                if word.eq(name) {
                    return Err(format!("\"{}\" is a protected keyword", name));
                }
            }

            let keys = fields.keys();
            
            for key in keys {
                for word in KEYWORDS {
                    if word.eq(key) {
                        return Err(
                                format!("\"{}\" is a protected keyword", key));
                    }
                }
            }

            enviornment.insert(name.to_string(), 
                               Value::Object{
                                    name: name.to_string(), 
                                    fields: fields.clone(), 
                                    init: params.init.clone(), 
                                    methods: params.methods.clone(),
                                    parent_class: params.parent.clone()});


            Ok(Value::Null)
        },
        Expression::FieldAccess { name, field } => {
            let res = enviornment.get(name);
            let Some(val) = res else { 
                return Err(format!("{} is undefined", name)) 
            };
            
            let obj_fields = match val {
                Value::Object { name: _, fields, .. } => fields.clone(),
                _ => return Err("Can only access fields on objects".to_string())
            };

            for (field_name, field_data) in obj_fields {
                if field_name == *field 
                && (!field_data.is_private || name == "this") {
                    return Ok(field_data.value);
                }
            }

            Err("Cannot access private fields!".to_string())
        },
        Expression::ObjectCreation { class_name, arguments } => {
            let Some(res) = enviornment.get(class_name) else {
                return Err(format!("{} is undefined", class_name))
            };
            
            let Value::Object { name, fields, init, 
                                methods, parent_class } = res 
                else { return Err(format!("{} is not a class", class_name)) };

            if init.name.is_none() {
                return Ok(Value::Object{name: name.clone(), 
                                        fields: fields.clone(), 
                                        init: init.clone(), 
                                        methods: methods.clone(),
                                        parent_class: parent_class.clone()});
            }

            let init_args = match &init.arguments {
                Some(s) => s.clone(),
                None => vec![]
            };

            if init_args.len() != arguments.len() {
                return Err(format!("Expected {} arguments but got {}", 
                                    init_args.len(), arguments.len()))
            }

            let init_statements = match &init.statements {
                Some(s) => s.clone(),
                None => vec![]
            };

            let mut local_env = HashMap::<String, Value>::new();
            
            let mut parent_object: Option<Value> = None;
            if parent_class.is_some(){
                let p_name = parent_class.clone().unwrap();

                let res = enviornment.get(&p_name);
                let Some(val) = res else {
                    return Err(format!("{} is not defined", p_name));
                };

                let p_obj= match val {
                    Value::Object{ .. } => val,
                    _ => return Err(format!("{} is not an object", p_name)) 
                };

                local_env.insert("super".to_string(), p_obj.clone());
                parent_object = Some(p_obj.clone());
            }

            for (name, exp) in init_args.iter().zip(arguments.iter()) {
                let val = eval_expression(&mut enviornment.clone(), 
                                      exp, importing)?;
                local_env.insert(name.to_string(), val);
            }

            for (name, data) in fields {
                local_env.insert(name.to_string(), data.value.clone());
            }

            for statement in init_statements {
                eval_statement(&mut local_env, &statement, importing)?;
            }
            
            let mut updated_fields = HashMap::<String, ClassField>::new();

            for (field, data) in fields {
                let Some(updated_field) = local_env.get(field) else {
                    return Err(format!(
                            "An error occured when initalizing {}", field))
                };
                let mut new_data = data.clone();

                new_data.value = updated_field.clone();

                updated_fields.insert(field.to_string(), new_data);
            }

            if parent_object.is_some() {
                let obj = parent_object.unwrap();

                let Value::Object{ ref fields, .. } = obj else {
                    return Err("Dev error: Non-object assigned to parent 
                                eval:Expression:ObjectCreation".to_string())
                };

                for (field, data) in fields {
                    let Some(updated_field) = local_env.get(field) else {
                        return Err(format!(
                                "An error occured when initalizing {}", field))
                    };
                    let mut new_data = data.clone();

                    new_data.value = updated_field.clone();

                    updated_fields.insert(field.to_string(), new_data);
                }
                let parent_field = ClassField {
                    is_private: true,
                    value: obj.clone(),
                };

                updated_fields.insert("super".to_string(),parent_field);
            }
            

            Ok(Value::Object{name: name.clone(), fields: updated_fields, 
                             init: init.clone(), methods: methods.clone(),
                             parent_class: parent_class.clone()})
        },
        Expression::MethodCall { name, method, arguments  } => {
            if name == "super" {

                let Some(res) = enviornment.get("super") else {
                    return Err("super used with no parent".to_string());
                };

                let Value::Object{methods, fields, ..} = res else {
                    return Err("dev error, super not blacklisted".to_string());
                };

                for (name, data) in methods {
                    if name == method{
                        let method_args = data.arguments.clone();
                        if arguments.len() != method_args.len() {
                            return Err(format!("Expected {} arguments, got {}", 
                                            method_args.len(), arguments.len()))
                        }
                        let mut local_env = HashMap::<String, Value>::new();
                        for (name, exp) 
                            in method_args.iter().zip(arguments.iter()) {
                            let val = eval_expression(
                                &mut enviornment.clone(), exp, importing)?;
                            local_env.insert(name.to_string(), val);
                        }

                        for name in fields.keys() {
                            let Some(val) = enviornment.get(name) else {
                                return Err(
                                    "Dev error, undefined field".to_string());
                            };
                            local_env.insert(name.to_string(), 
                                             val.clone());
                        }

                        insert_builtins(&mut local_env);

                        for statement in &data.statements {
                            eval_statement(&mut local_env, 
                                            statement, importing)?;
                        }

                        if data.return_exp.is_some() {
                            let exp = data.return_exp.clone().unwrap();
                            return eval_expression(&mut local_env, 
                                        &exp, importing)
                        }

                        return Ok(Value::Null);
                    }
                }

                return Err(format!("{} is not defined or is private", method));
            }

            let Some(object) = enviornment.get(name) else {
                return Err(format!("{} is not defined", name))
            };

            let Value::Object{ fields: object_fields, 
                               init: _init, methods: object_methods , ..} 
                               = object else {
                return Err(format!("{} is not an object", name))
            };

            for (m_name, method_data) in object_methods {
                if m_name == method 
                    && (!method_data.is_private || name == "this") {
                    let obj_arguments = method_data.arguments.clone();

                    if obj_arguments.len() != arguments.len() {
                        return Err(format!("Expected {} arguments, got {}", 
                                            obj_arguments.len(),
                                            arguments.len()));
                    }

                    let statements = method_data.statements.clone();
                    let return_exp = method_data.return_exp.clone();

                    let mut local_env = HashMap::<String, Value>::new();
                    insert_builtins(&mut local_env);
                    local_env.insert("this".to_string(), object.clone());

                    let mut env_clone = enviornment.clone();

                    for (name, argument) 
                        in obj_arguments.iter().zip(arguments.iter()) {
                            
                        let res = eval_expression(
                            &mut env_clone, argument, importing)?;
                        local_env.insert(name.to_string(), res);
                    }

                    for (name, data) in object_fields {
                        local_env.insert(name.to_string(), data.clone().value);
                    }

                    for statement in statements {
                        eval_statement(&mut local_env, &statement, importing)?;
                    }

                    if return_exp.is_none() {
                        return Ok(Value::Null)
                    }

                    let return_val = eval_expression(
                        &mut local_env, &return_exp.unwrap(), importing)?;
                    return Ok(return_val)
                }
            };

            Err(format!("{} is not a valid method or is private", method))
        }
        //_=> Err(format!("unhandled expression: {:?}", expression)),
    }
}

fn eval_expressions(enviornment: &mut HashMap<String, Value>, 
                    expressions: &Vec<Expression>, 
                    importing: bool) -> Result<Vec<Value>, String> {
        let mut vals = vec![];

        for expression in expressions {
            match eval_expression(enviornment, expression, importing) {
                Ok(v) => vals.push(v),
                Err(e) => return Err(e),
            }
        }

        Ok(vals)
}

fn operate(operator: &Operator, lhs: &Value, rhs: &Value) 
    -> Result<Value, String>{
        match operator {
            Operator::Plus => Ok(lhs + rhs),
            Operator::Minus => Ok(lhs - rhs),
            Operator::Times => Ok(lhs * rhs),
            Operator::Divide => Ok(lhs / rhs),
            Operator::LessThan => Ok(Value::Bool{b: lhs < rhs}),
            Operator::GreaterThan => Ok(Value::Bool{b: lhs > rhs}),
            Operator::Equal => Ok(Value::Bool{b: lhs == rhs}),
            Operator::NotEqual => Ok(Value::Bool{b: lhs != rhs}),
        }
}
              
fn insert_builtins(env: &mut HashMap<String, Value>){
    env.insert("println".to_string(), 
        Value::Function{name: "println".to_string(), f: println_});
    
    env.insert("print".to_string(), 
        Value::Function{name: "print".to_string(), f: print_});

    env.insert("range".to_string(), 
        Value::Function{name: "range".to_string(), f: range});

    env.insert("range_step".to_string(), 
        Value::Function{name: "range_step".to_string(), f: range_step});
}