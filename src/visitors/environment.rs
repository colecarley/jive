use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Environment<T: Clone> {
    values: HashMap<String, T>,
    enclosing: Option<Rc<RefCell<Environment<T>>>>,
}

impl<T: Clone> Environment<T> {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn declare_global(&mut self, identifier: String, value: T) {
        if let Some(enclosing) = &mut self.enclosing {
            enclosing
                .borrow_mut()
                .declare_global(identifier.clone(), value.clone());
        }
        self.values.insert(identifier, value);
    }

    pub fn get(&self, identifier: String) -> T {
        if self.values.contains_key(&identifier) {
            return self.values.get(&identifier).unwrap().clone();
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().get(identifier),
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
                Some(enclosing) => enclosing.borrow_mut().assign(identifier, value),
                None => panic!("Undefined variable {}", identifier),
            }
        }
    }

    pub fn enclose(&mut self, enclosing: Rc<RefCell<Environment<T>>>) {
        self.enclosing = Some(enclosing.clone());
    }

    pub fn get_enclosing(&self) -> Rc<RefCell<Environment<T>>> {
        self.enclosing.clone().expect("No enclosing environment")
    }
}
