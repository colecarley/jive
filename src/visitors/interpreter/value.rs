use super::callable::{BuiltIn, Function};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    BuiltIn(BuiltIn),
    Function(Function),
    List(Vec<Value>),
    Iter(Vec<Value>),
    Nil,
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            (Value::String(left), Value::String(right)) => Value::String(left + &right),
            _ => panic!("Operands must be numbers"),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
            _ => panic!("Operands must be numbers"),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
            _ => panic!("Operands must be numbers"),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left / right),
            _ => panic!("Operands must be numbers"),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Number(right) => Value::Number(-right),
            _ => panic!("Unary operator - can only be applied to numbers"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::Boolean(left), Value::Boolean(right)) => left == right,
            (Value::String(left), Value::String(right)) => left == right,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => left.partial_cmp(right),
            _ => panic!("Operands must be numbers"),
        }
    }
}

impl std::ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Value::Boolean(right) => Value::Boolean(!right),
            _ => panic!("Unary operator ! can only be applied to booleans"),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(number) => number.to_string(),
            Value::Boolean(boolean) => boolean.to_string(),
            Value::String(string) => string.clone(),
            Value::BuiltIn(_) => format!("<native funk>"),
            Value::Function(_) => format!("<funk>"),
            Value::Iter(iter) => format!(
                "Iter [{}]",
                iter.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::List(list) => format!(
                "[{}]",
                list.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Nil => "nil".to_string(),
        }
    }
}
