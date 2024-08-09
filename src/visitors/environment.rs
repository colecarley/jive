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

    pub fn declare(&mut self, identifier: String, value: T) {
        self.values.insert(identifier, value);
    }

    pub fn assign(&mut self, identifier: String, value: T) {
        if self.values.contains_key(&identifier) {
            self.values.insert(identifier, value);
        } else {
            match &mut self.enclosing {
                Some(enclosing) => enclosing.assign(identifier, value),
                None => panic!("Undefined variable {}", identifier),
            }
        }
    }

    pub fn enclose(&mut self, enclosing: &Box<Environment<T>>) {
        self.enclosing = Some(enclosing.clone());
    }

    pub fn get_enclosing(&self) -> Box<Environment<T>> {
        self.enclosing.clone().expect("No enclosing environment")
    }
}
