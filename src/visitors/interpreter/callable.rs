use std::{cell::RefCell, rc::Rc};

use crate::{
    parser::{accept::Accept, statement::FunctionDeclaration},
    visitors::environment::Environment,
};

use super::{value::Value, Interpreter};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub arity: usize,
    pub declaration: FunctionDeclaration,
    pub closure: Rc<RefCell<Environment<Value>>>,
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
        let new_environment = Rc::new(RefCell::new(Environment::new()));
        new_environment.borrow_mut().enclose(self.closure.clone());

        for (parameter, argument) in self.declaration.parameters.iter().zip(arguments.iter()) {
            new_environment
                .borrow_mut()
                .declare(parameter.lexeme.clone(), argument.clone());
        }

        let previous_environment = interpreter.environment.clone();
        interpreter.environment = new_environment.clone();
        let (result, _) = self.declaration.body.accept(interpreter);

        interpreter.environment = previous_environment;

        result
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BuiltIn {
    pub arity: Option<usize>,
    pub function: fn(&mut Interpreter, &mut Vec<Value>) -> Value,
}

impl BuiltIn {
    pub fn new(
        arity: Option<usize>,
        function: fn(&mut Interpreter, &mut Vec<Value>) -> Value,
    ) -> Self {
        BuiltIn { arity, function }
    }
}

impl Callable for BuiltIn {
    fn call(&self, interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
        (self.function)(interpreter, arguments)
    }
}

pub fn clock(_interpreter: &mut Interpreter, _arguments: &mut Vec<Value>) -> Value {
    // arity is Some(0)
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1_000 + since_the_epoch.subsec_millis() as u64;

    Value::Number(in_ms as f64)
}

pub fn println(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is None
    for argument in arguments {
        match argument {
            Value::Number(number) => print!("{}", number),
            Value::Boolean(boolean) => print!("{}", boolean),
            Value::String(string) => print!("{}", string),
            Value::BuiltIn(_) => print!("<native funk>"),
            Value::Function(_) => print!("<funk>"),
            Value::Record(record) => print!(
                "{{{}}}",
                record
                    .keys()
                    .map(|k| { format!("{}:{},", k, record.get(k).unwrap().to_string()) })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Iter(iter) => print!(
                "Iter [{}]",
                iter.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::List(list) => print!(
                "[{}]",
                list.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Nil => println!("nil"),
        }
    }
    println!();

    Value::Nil
}

pub fn input(_interpreter: &mut Interpreter, _arguments: &mut Vec<Value>) -> Value {
    // arity is Some(0)
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Value::String(input.trim().to_string())
}

pub fn iter(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(1)
    let value = &arguments[0];

    match value {
        Value::String(string) => Value::Iter(
            string
                .chars()
                .map(|c| Value::String(c.to_string()))
                .collect(),
        ),
        Value::List(list) => Value::Iter(list.clone()),
        _ => panic!("Input must be of type either string or list"),
    }
}

pub fn range_max(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(1)
    let max = &arguments[0];

    match max {
        Value::Number(max) => {
            if max.is_sign_negative() {
                panic!("Must pass a positive number to range function");
            } else {
                Value::Iter(
                    (0..(*max as i64))
                        .map(|v| Value::Number(v as f64))
                        .collect::<Vec<Value>>(),
                )
            }
        }
        _ => panic!("Must pass a number to range function"),
    }
}

pub fn range_min_max(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(2)
    let min = &arguments[0];
    let max = &arguments[1];

    if let Value::Number(min) = min {
        if min.is_sign_negative() {
            panic!("Must pass a positive number to range function");
        }

        if let Value::Number(max) = max {
            if min >= max {
                panic!("First argument must be smaller than the second argument");
            }

            Value::Iter(
                ((*min as i64)..(*max as i64))
                    .map(|v| Value::Number(v as f64))
                    .collect::<Vec<Value>>(),
            )
        } else {
            panic!("Must pass a number to range function")
        }
    } else {
        panic!("Must pass a number to range function")
    }
}

pub fn range_min_max_skip(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(2)
    let min = &arguments[0];
    let max = &arguments[1];
    let skip = &arguments[2];

    if let Value::Number(min) = min {
        if min.is_sign_negative() {
            panic!("Must pass a positive number to range function");
        }

        if let Value::Number(max) = max {
            if min >= max {
                panic!("First argument must be smaller than the second argument");
            }

            if let Value::Number(skip) = skip {
                Value::Iter(
                    ((*min as i64)..(*max as i64))
                        .step_by((*skip as i64) as usize)
                        .map(|v| Value::Number(v as f64))
                        .collect::<Vec<Value>>(),
                )
            } else {
                panic!("Must pass a number to range function")
            }
        } else {
            panic!("Must pass a number to range function")
        }
    } else {
        panic!("Must pass a number to range function")
    }
}

pub fn len(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(1)
    Value::Number(match &arguments[0] {
        Value::List(list) => list.len() as f64,
        Value::String(string) => string.len() as f64,
        _ => panic!("Must pass either a list or a string to len function"),
    })
}

pub fn push(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(2)
    let value = arguments[1].clone();

    match &mut arguments[0] {
        Value::List(list) => {
            list.push(value);
            Value::List(list.to_vec())
        }
        _ => panic!("Must pass either a list to push function"),
    }
}

pub fn to_number(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(1)

    Value::Number(match &arguments[0] {
        Value::String(string) => string.parse().expect("Could not parse string"),
        _ => panic!("Must pass a string to to_number function"),
    })
}

pub fn type_of(_interpreter: &mut Interpreter, arguments: &mut Vec<Value>) -> Value {
    // arity is Some(1)

    Value::String(match &arguments[0] {
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Boolean(_) => "boolean".to_string(),
        Value::BuiltIn(_) => "builtin function".to_string(),
        Value::Function(_) => "function".to_string(),
        Value::List(_) => "list".to_string(),
        Value::Iter(_) => "iter".to_string(),
        Value::Record(_) => "record".to_string(),
        Value::Nil => "nil".to_string(),
    })
}
