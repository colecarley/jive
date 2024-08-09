use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment<T: Clone> {
    values: HashMap<String, T>,
    enclosing: Option<Box<Environment<T>>>,
}

impl<T: Clone> Environment<T> {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn get(&self, identifier: String) -> T {
        if self.values.contains_key(&identifier) {
            return self.values.get(&identifier).unwrap().clone();
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.get(identifier),
                None => panic!("Undefined variable {}", identifier),
            }
        }
    }

    pub fn insert(&mut self, identifier: String, value: T) {
        self.values.insert(identifier, value);
    }

    pub fn enclose(&mut self, enclosing: &Environment<T>) {
        self.enclosing = Some(Box::new(enclosing.clone()));
    }
}
