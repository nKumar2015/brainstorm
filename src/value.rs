use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign };

use crate::ast::{Expression, Statement};

#[derive(Debug)]
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
    Function{name: String, f: fn(Vec<Value>) -> Result<Value, String>},
    #[allow(dead_code)]
    UserDefFunction{name: String, statements: Vec<Statement>, 
        arguments: Vec<String>, return_expression: Option<Expression> }
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
                let item = e[self.index].clone();
                self.index += 1;
                Some(item)
            },
            Value::Str { s } => {
                let mut chars = s.chars();
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
                                    arguments, return_expression 
                                  } => Self::UserDefFunction { 
                    name: name.clone(), statements: statements.clone(), 
                    arguments: arguments.clone(), 
                    return_expression: return_expression.clone() },
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
