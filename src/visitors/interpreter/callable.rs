use std::{cell::RefCell, rc::Rc};

use crate::{
    parser::{accept::Accept, statement::FunctionDeclaration},
    visitors::environment::Environment,
};

use super::{value::Value, Interpreter};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub arity: usize,
    pub declaration: FunctionDeclaration,
    pub closure: Rc<RefCell<Environment<Value>>>,
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let new_environment = Rc::new(RefCell::new(Environment::new()));
        new_environment.borrow_mut().enclose(self.closure.clone());

        for (parameter, argument) in self.declaration.parameters.iter().zip(arguments.iter()) {
            new_environment
                .borrow_mut()
                .declare(parameter.lexeme.clone(), argument.clone());
        }

        interpreter.environment = new_environment.clone();
        let (result, _) = self.declaration.body.accept(interpreter);

        interpreter.environment = new_environment.borrow_mut().get_enclosing();

        result
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BuiltIn {
    pub arity: Option<usize>,
    pub function: fn(&mut Interpreter, Vec<Value>) -> Value,
}

impl BuiltIn {
    pub fn new(arity: Option<usize>, function: fn(&mut Interpreter, Vec<Value>) -> Value) -> Self {
        BuiltIn { arity, function }
    }
}

impl Callable for BuiltIn {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        (self.function)(interpreter, arguments)
    }
}

pub fn clock(_interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Value {
    // arity is Some(0)
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1_000 + since_the_epoch.subsec_millis() as u64;

    Value::Number(in_ms as f64)
}

pub fn println(_interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
    // arity is None
    for argument in arguments {
        match argument {
            Value::Number(number) => print!("{}", number),
            Value::Boolean(boolean) => print!("{}", boolean),
            Value::String(string) => print!("{}", string),
            Value::BuiltIn(callable) => print!("{:?}", callable),
            Value::Function(function) => print!("{:?}", function),
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

pub fn input(_interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Value {
    // arity is Some(0)
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Value::String(input)
}

pub fn iter(_interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
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
