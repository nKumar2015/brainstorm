use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign };

use crate::ast::{Expression, Statement, ClassInitDef, ClassField, ClassMethod};

#[derive(Debug)]
pub enum Value {
    Null,
    Int{v: i32},
    Str{s: String},
    Bool{b: bool},
    Float{f: f64},
    Char{c: char},
    List{e: Vec<Value>},
    Function{name: String, f: fn(Vec<Value>) -> Result<Value, String>},
    UserDefFunction{name: String, statements: Vec<Statement>, 
        arguments: Vec<String>, return_expression: Option<Expression>},
    Object{name: String, fields: HashMap<String, ClassField>, 
             init: ClassInitDef, methods: HashMap<String, ClassMethod>}
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_print ;
        match self {
            Value::Null 
                => to_print = String::from("Null"),
            Value::Int { v } 
                => to_print = format!("{}", v),
            Value::Str { s } 
                => to_print = String::from(s),
            Value::Bool { b } 
                => to_print = format!("{}", b),
            Value::Float { f } 
                => to_print = format!("{}", f),
            Value::Char { c } 
                => to_print = format!("{}", c),
            Value::List { e } => {
                to_print = String::new();
                to_print.push('[');
                for (idx, val) in e.iter().enumerate() {
                    to_print.push_str(&val.to_string());
                    if idx != e.len() - 1 {
                        to_print.push_str(", ");
                    }
                }
                to_print.push(']');
            },
            Value::Function { name, .. } 
                => to_print = format!("Function \"{}\"", name),
            Value::UserDefFunction { name, .. } 
                => to_print = format!("Function \"{}\"", name),
            Value::Object { name,.. } 
                => to_print = format!("Class \"{}\"", name)
        };
        write!(f, "{}", to_print)
    }
}

pub struct ValueIterator{
    pub value: Value,
    index: usize,
}

impl IntoIterator for Value {
    type Item = Value;

    type IntoIter = ValueIterator;

    fn into_iter(self) -> Self::IntoIter {
        match &self {
            Value::List { .. } => ValueIterator{
                value: self, 
                index: 0
            },
            Value::Str { .. } => ValueIterator{
                value: self, 
                index: 0
            },
            _ => {
                ValueIterator{
                    value: Value::Null, 
                    index: 0
                }
            }
        }
    }
}

impl Iterator for ValueIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let val = &self.value;
        match val {
            Value::List { e } => {
                if self.index >= e.len() {
                    return None;
                }
                let item = e[self.index].clone();
                self.index += 1;
                Some(item)
            },
            Value::Str { s } => {
                let mut chars = s.chars();
                let length = chars.clone().count();
                if self.index >= length {
                    return None;
                }
                let item = chars.nth(self.index)
                    .expect("Error getting char from string");
                self.index += 1;
                Some(Value::Char{c: item})
            },
            _ => None 
        }
    }
}

impl Clone for ValueIterator{
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), index: self.index }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null)
                => true,
            (Value::Int { v: l_v }, Value::Int { v: r_v }) 
                => l_v == r_v,
            (Value::Str { s: l_s },Value::Str { s: r_s }) 
                => l_s == r_s,
            (Value::Bool { b: l_b }, Value::Bool { b: r_b }) 
                => l_b == r_b,
            (Value::Float { f: l_f }, Value::Float { f: r_f }) 
                => l_f == r_f,
            (Value::Char { c: l_c }, Value::Char { c: r_c }) 
                => l_c == r_c,
            (Value::List { e: l_e }, Value::List { e: r_e }) => {
                for (lhs, rhs) in l_e.iter().zip(r_e) {
                    if lhs != rhs {
                        return false
                    }
                }
                true
            },
            (Value::Float { f }, Value::Int { v }) 
                => f64::from(*v) == *f,
            (Value::Int { v }, Value::Float { f })
                => f64::from(*v) == *f,
            _ 
                => core::mem::discriminant(self) 
                    == core::mem::discriminant(other),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Null, Value::Null) 
                => Ordering::Equal,
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => lv.cmp(rv),
            (Value::Float { f }, Value::Int { v }) => {
                if *f < f64::from(*v) {
                    return Ordering::Less
                }
                if *f > f64::from(*v) {
                    return Ordering::Greater
                }
                Ordering::Equal
            },
            (Value::Int { v }, Value::Float { f }) => {
                if f64::from(*v) < *f {
                    return Ordering::Less
                }
                if f64::from(*v) > *f{
                    return Ordering::Greater
                }
                Ordering::Equal
            },
            (Value::Str { s: ls }, Value::Str{ s: rs}) 
                => ls.cmp(rs),
            (Value::Bool { b: lb }, Value::Bool{ b: rb}) 
                => lb.cmp(rb),
            (Value::Float { f: lf }, Value::Float{ f: rf}) => {
                if lf < rf {
                    return Ordering::Less
                }
                if lf > rf {
                    return Ordering::Greater
                }
                Ordering::Equal
            },
            (Value::Char { c: lc }, Value::Char{ c: rc}) 
                => lc.cmp(rc),
            (Value::List{ .. }, Value::List{ .. } )
                => panic!("Cannot compare two lists!"),
            (Value::Function{ .. }, Value::Function{ .. } )
                => panic!("Cannot compare two functions!"),
            (Value::UserDefFunction{ .. }, Value::UserDefFunction{ .. } )
                => panic!("Cannot compare two functions!"),
            (Value::Object{ .. }, Value::Object{ .. } )
                => panic!("Cannot compare two classes!"),
            _ => panic!("Cannot compare these two types!")
        }    
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Value {}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Null 
                => Self::Null,
            Self::Int { v } 
                => Self::Int { v: *v },
            Self::Str { s } 
                => Self::Str { s: s.clone() },
            Self::Bool { b } 
                => Self::Bool { b: *b },
            Self::Float { f } 
                => Self::Float { f: *f },
            Self::Char { c } 
                => Self::Char { c: *c },
            Self::List { e } 
                => Self::List { e: e.clone() },
            Self::Function { name, f } 
                => Self::Function { name: name.clone(), f: *f },
            Self::UserDefFunction { name, statements, 
                                    arguments, return_expression } 
                => Self::UserDefFunction { 
                    name: name.clone(), 
                    statements: statements.clone(), 
                    arguments: arguments.clone(), 
                    return_expression: return_expression.clone() },
            Self::Object { name, fields, init, methods } 
                => Self::Object{name: name.clone(), 
                                  fields: fields.clone(), 
                                  init: init.clone(), 
                                  methods: methods.clone()}
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv + rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf + rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(v) + f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f + f64::from(v)},
            _ => Value::Null
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        *self = match (&self, other) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: *lv + rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: *lf + rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) + f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: *f + f64::from(v)},
            _ => Value::Null
        };
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv - rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf - rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(v) - f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f - f64::from(v)},
            _ => Value::Null
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, other: Self) {
        *self = match (&self, other) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: *lv - rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: *lf - rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) - f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: *f - f64::from(v)},
            _ => Value::Null
        };
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv / rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf / rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(v) / f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f / f64::from(v)},
            _ => Value::Null
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, other: Self) {
        *self = match (&self, other) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: *lv / rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: *lf / rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) / f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: *f / f64::from(v)},
            _ => Value::Null
        };
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv * rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf * rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(v) * f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f * f64::from(v)},
            _ => Value::Null
        }
    }
}

impl MulAssign for Value {
    fn mul_assign(&mut self, other: Self) {
        *self = match (&self, other) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: *lv + rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: *lf + rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) * f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: *f * f64::from(v)},
            _ => Value::Null
        };
    }
}

impl Add for &Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv + rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf + rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) + f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f + f64::from(*v)},
            _ => Value::Null
        }
    }
}

impl Sub for &Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv - rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf - rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) - f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f - f64::from(*v)},
            _ => Value::Null
        }
    }
}

impl Div for &Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv / rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf / rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) / f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f / f64::from(*v)},
            _ => Value::Null
        }
    }
}

impl Mul for &Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int { v: lv }, Value::Int { v: rv }) 
                => Value::Int{v: lv * rv},
            (Value::Float { f: lf }, Value::Float { f: rf }) 
                => Value::Float{f: lf * rf},
            (Value::Int { v }, Value::Float { f }) 
                => Value::Float{f: f64::from(*v) * f},
            (Value::Float { f }, Value::Int { v }) 
                => Value::Float{f: f * f64::from(*v)},
            _ => Value::Null
        }
    }
}
