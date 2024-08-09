use std::collections::HashMap;

use super::Value;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn get(&self, identifier: String) -> Value {
        if self.values.contains_key(&identifier) {
            return self.values.get(&identifier).unwrap().clone();
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.get(identifier),
                None => panic!("Undefined variable {}", identifier),
            }
        }
    }

    pub fn insert(&mut self, identifier: String, value: Value) {
        self.values.insert(identifier, value);
    }

    pub fn enclose(&mut self, enclosing: &Environment) {
        self.enclosing = Some(Box::new(enclosing.clone()));
    }
}
