use super::{value::Value, Interpreter};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
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
