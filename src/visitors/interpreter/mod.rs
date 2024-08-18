use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    parser::{
        accept::Accept,
        expression::{
            And, Assignment, Call, Comparison, Equality, Factor, IfExpression, Index,
            IndexAssignment, List, MapIndex, MapIndexAssignment, Or, Primary, Record, Term, Unary,
        },
        statement::{
            Block, ExpressionStatement, For, FunctionDeclaration, IfStatement, PrintStatement,
            Return, Statement, WhileStatement,
        },
    },
    token::TokenType,
};

pub mod callable;
pub mod value;

use super::environment::Environment;
use callable::{BuiltIn, Callable, Function};
use value::Value;

pub struct Interpreter {
    environment: Rc<RefCell<Environment<Value>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Rc::new(RefCell::new(Environment::<Value>::new()));

        environment.borrow_mut().declare_global(
            "clock".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(0), callable::clock))),
        );
        environment.borrow_mut().declare_global(
            "println".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(None, callable::println))),
        );
        environment.borrow_mut().declare_global(
            "input".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(0), callable::input))),
        );
        environment.borrow_mut().declare_global(
            "iter".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(1), callable::iter))),
        );
        environment.borrow_mut().declare_global(
            "range_to".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(1), callable::range_max))),
        );
        environment.borrow_mut().declare_global(
            "range".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(2), callable::range_min_max))),
        );
        environment.borrow_mut().declare_global(
            "range_skip".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(
                Some(3),
                callable::range_min_max_skip,
            ))),
        );
        environment.borrow_mut().declare_global(
            "len".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(1), callable::len))),
        );
        environment.borrow_mut().declare_global(
            "push".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(2), callable::push))),
        );
        environment.borrow_mut().declare_global(
            "to_number".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(1), callable::to_number))),
        );
        environment.borrow_mut().declare_global(
            "type_of".to_string(),
            Value::BuiltIn(Box::new(BuiltIn::new(Some(1), callable::type_of))),
        );

        Interpreter { environment }
    }

    pub fn evaluate(&mut self, statements: &Vec<Statement>) -> Value {
        for statement in statements {
            let (_, ret) = statement.accept(self);
            if ret {
                panic!("Return keyword should not be used outside of a function body");
            }
        }

        Value::Nil
    }
}

impl super::Visitor for Interpreter {
    type Output = (Value, bool);

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
        let (value, _) = assignment.value.accept(self);
        self.environment
            .borrow_mut()
            .assign(assignment.identifier.lexeme.clone(), value.clone());

        (value, false)
    }

    fn visit_equality(&mut self, equality: &Equality) -> Self::Output {
        let (left, _) = equality.left.accept(self);
        let (right, _) = equality.right.accept(self);

        (
            match equality.operator.token_type {
                TokenType::EqualEqual => Value::Boolean(Box::new(left == right)),
                TokenType::BangEqual => Value::Boolean(Box::new(left != right)),
                _ => panic!("Unexpected token type"),
            },
            false,
        )
    }

    fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output {
        let (left, _) = comparison.left.accept(self);
        let (right, _) = comparison.right.accept(self);

        (
            match comparison.operator.token_type {
                TokenType::Greater => Value::Boolean(Box::new(left > right)),
                TokenType::GreaterEqual => Value::Boolean(Box::new(left >= right)),
                TokenType::Less => Value::Boolean(Box::new(left < right)),
                TokenType::LessEqual => Value::Boolean(Box::new(left <= right)),
                _ => panic!("Unexpected token type"),
            },
            false,
        )
    }

    fn visit_term(&mut self, term: &Term) -> Self::Output {
        let (left, _) = term.left.accept(self);
        let (right, _) = term.right.accept(self);
        (
            match term.operator.token_type {
                TokenType::Plus => left + right,
                TokenType::Minus => left - right,
                _ => panic!("Unexpected token type"),
            },
            false,
        )
    }

    fn visit_factor(&mut self, factor: &Factor) -> Self::Output {
        let (left, _) = factor.left.accept(self);
        let (right, _) = factor.right.accept(self);
        (
            match factor.operator.token_type {
                TokenType::Star => left * right,
                TokenType::Slash => left / right,
                _ => panic!("Unexpected token type"),
            },
            false,
        )
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        let (value, _) = unary.right.accept(self);
        (
            match unary.operator.token_type {
                TokenType::Minus => -value,
                TokenType::Bang => {
                    Value::Boolean(Box::new(value == Value::Boolean(Box::new(false))))
                }
                _ => panic!("Unexpected token type"),
            },
            false,
        )
    }

    fn visit_primary(&mut self, primary: &Primary) -> Self::Output {
        (
            match primary.value.token_type {
                TokenType::Number => Value::Number(Box::new(primary.value.lexeme.parse().unwrap())),
                TokenType::Boolean => Value::Boolean(Box::new(primary.value.lexeme == "true")),
                TokenType::String => Value::String(Box::new(primary.value.lexeme.clone())),
                TokenType::Identifier => self
                    .environment
                    .borrow_mut()
                    .get(primary.value.lexeme.clone()),
                TokenType::Nil => Value::Nil,
                _ => panic!("Unexpected token type"),
            },
            false,
        )
    }

    fn visit_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Self::Output {
        expression_statement.expression.accept(self);

        (Value::Nil, false)
    }

    fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output {
        let (value, _) = print_statement.expression.accept(self);

        match value {
            Value::Number(number) => println!("{}", number),
            Value::Boolean(boolean) => println!("{}", boolean),
            Value::String(string) => println!("{}", string),
            Value::BuiltIn(_) => println!("<native funk>"),
            Value::Function(_) => println!("<funk>"),
            Value::Record(record) => println!(
                "{{{}}}",
                record
                    .borrow()
                    .keys()
                    .map(|k| { format!("{}:{}", k, record.borrow().get(k).unwrap().to_string()) })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Iter(iter) => println!(
                "Iter [{}]",
                iter.borrow()
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::List(list) => println!(
                "[{}]",
                list.borrow()
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Nil => println!("nil"),
        }

        (Value::Nil, false)
    }

    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &crate::parser::statement::VariableDeclaration,
    ) -> Self::Output {
        if let Some(expression) = &variable_declaration.expression {
            let (value, _) = expression.accept(self);
            self.environment
                .borrow_mut()
                .declare(variable_declaration.identifier.lexeme.clone(), value);

            return (Value::Nil, false);
        }
        self.environment
            .borrow_mut()
            .declare(variable_declaration.identifier.lexeme.clone(), Value::Nil);

        (Value::Nil, false)
    }

    fn visit_block(&mut self, block: &Block) -> Self::Output {
        let new_environment = Rc::new(RefCell::new(Environment::new()));
        new_environment
            .borrow_mut()
            .enclose(self.environment.clone());
        self.environment = new_environment.clone();

        for statement in &block.statements {
            let (result, ret) = statement.accept(self);
            if ret {
                return (result, ret);
            }
        }

        self.environment = new_environment.borrow_mut().get_enclosing();
        (Value::Nil, false)
    }

    fn visit_if_statement(&mut self, if_statement: &IfStatement) -> Self::Output {
        let (condition, _) = if_statement.condition.accept(self);
        if condition == Value::Boolean(Box::new(true)) {
            let (result, ret) = if_statement.then_branch.accept(self);
            if ret {
                return (result, ret);
            }
            return (Value::Nil, false);
        }

        if let Some(else_branch) = &if_statement.else_branch {
            else_branch.accept(self);
        }

        (Value::Nil, false)
    }

    fn visit_if_expression(&mut self, if_expression: &IfExpression) -> Self::Output {
        let (condition, _) = if_expression.condition.accept(self);

        if condition == Value::Boolean(Box::new(true)) {
            return if_expression.then_branch.accept(self);
        }

        if_expression.else_branch.accept(self)
    }

    fn visit_and(&mut self, and: &And) -> Self::Output {
        let (left, _) = and.left.accept(self);

        if left == Value::Boolean(Box::new(false)) {
            return (Value::Boolean(Box::new(false)), false);
        }

        and.right.accept(self)
    }

    fn visit_or(&mut self, or: &Or) -> Self::Output {
        let (left, _) = or.left.accept(self);

        if left == Value::Boolean(Box::new(true)) {
            return (Value::Boolean(Box::new(true)), false);
        }

        or.right.accept(self)
    }

    fn visit_while_statement(&mut self, while_statement: &WhileStatement) -> Self::Output {
        loop {
            let (condition, _) = while_statement.condition.accept(self);
            if condition == Value::Boolean(Box::new(false)) {
                break;
            }

            let (result, ret) = while_statement.body.accept(self);
            if ret {
                return (result, ret);
            }
        }

        (Value::Nil, false)
    }

    fn visit_call(&mut self, call: &Call) -> Self::Output {
        let (callee, _) = call.identifier.accept(self);

        let mut arguments = Vec::new();

        for argument in &call.arguments {
            let (arg, _) = argument.accept(self);
            arguments.push(arg);
        }

        match callee {
            Value::BuiltIn(callable) => {
                if callable.arity.is_none() {
                    return (callable.call(self, &mut arguments), false);
                }

                let arity = callable.arity.unwrap();
                if arguments.len() != arity {
                    panic!("Expected {} arguments but got {}", arity, arguments.len());
                }

                return (callable.call(self, &mut arguments), false);
            }
            Value::Function(function) => (function.call(self, &mut arguments), false),
            _ => panic!("Can only call functions"),
        }
    }

    fn visit_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Self::Output {
        let identifier = function_declaration.identifier.lexeme.clone();

        let function = Function {
            declaration: function_declaration.clone(),
            arity: function_declaration.parameters.len(),
            closure: if self.environment.borrow_mut().has_enclosing() {
                Rc::new(RefCell::new(self.environment.borrow_mut().clone()))
            } else {
                Rc::clone(&self.environment)
            },
        };

        let value = Value::Function(Box::new(function));

        self.environment.borrow_mut().declare(identifier, value);

        (Value::Nil, false)
    }

    fn visit_return(&mut self, return_statement: &Return) -> Self::Output {
        match return_statement.value {
            Some(ref value) => {
                let (result, _) = value.accept(self);
                return (result, true);
            }
            None => (Value::Nil, true),
        }
    }

    fn visit_with_statement(
        &mut self,
        with_statement: &crate::parser::statement::With,
    ) -> Self::Output {
        let new_environment = Rc::new(RefCell::new(Environment::new()));
        new_environment
            .borrow_mut()
            .enclose(self.environment.clone());

        let identifier = with_statement.identifier.lexeme.clone();
        let (value, _) = with_statement.value.accept(self);
        new_environment.borrow_mut().declare(identifier, value);

        self.environment = new_environment.clone();
        with_statement.body.accept(self);

        self.environment = new_environment.borrow_mut().get_enclosing();
        (Value::Nil, false)
    }

    fn visit_list(&mut self, list: &List) -> Self::Output {
        (
            Value::List(Rc::new(RefCell::new(
                list.values
                    .iter()
                    .map(|v| {
                        let (result, _) = v.accept(self);
                        result
                    })
                    .collect::<Vec<Value>>(),
            ))),
            false,
        )
    }

    fn visit_for_statement(&mut self, for_statement: &For) -> Self::Output {
        let identifer = for_statement.identifier.lexeme.clone();
        let (iter, _) = for_statement.iter.accept(self);

        match iter {
            Value::Iter(iter) => {
                let iter = iter.borrow();
                for value in iter.iter() {
                    let new_environment = Rc::new(RefCell::new(Environment::new()));
                    new_environment
                        .borrow_mut()
                        .enclose(self.environment.clone());

                    new_environment
                        .borrow_mut()
                        .declare(identifer.clone(), value.clone());

                    self.environment = new_environment.clone();
                    for_statement.body.accept(self);

                    self.environment = new_environment.borrow_mut().get_enclosing();
                }
                (Value::Nil, false)
            }
            _ => panic!("Must use an Iter in the 'for' statement"),
        }
    }

    fn visit_index(&mut self, index: &Index) -> Self::Output {
        let (list, _) = index.list.accept(self);
        let (expression, _) = index.expression.accept(self);

        match list {
            Value::List(indexable) => match expression {
                Value::Number(number) => {
                    (indexable.borrow()[(*number as i32) as usize].clone(), false)
                }
                _ => panic!("Must use number to index into list or string"),
            },
            Value::String(indexable) => match expression {
                Value::Number(number) => (
                    Value::String(Box::new(
                        indexable.as_bytes()[(*number as i32) as usize].to_string(),
                    )),
                    false,
                ),
                _ => panic!("Must use number to index into list or string"),
            },
            _ => panic!("Can only index into list or string"),
        }
    }

    fn visit_record(&mut self, record: &Record) -> Self::Output {
        let mut map = HashMap::<String, Value>::new();

        for (key, value) in &record.key_values {
            let new_key = &key.lexeme;
            let (new_value, _) = value.accept(self);
            map.insert(new_key.to_string(), new_value);
        }

        (Value::Record(Rc::new(RefCell::new(map))), false)
    }

    fn visit_map_index(&mut self, map_index: &MapIndex) -> Self::Output {
        let name = map_index.key.lexeme.clone();
        let (map, _) = map_index.map.accept(self);

        match map {
            Value::Record(record) => (
                record
                    .borrow_mut()
                    .get(&name)
                    .expect("key does not exit in record")
                    .clone(),
                false,
            ),
            _ => panic!("Cannot dot index into non record type"),
        }
    }

    fn visit_map_index_assignment(
        &mut self,
        map_index_assignment: &MapIndexAssignment,
    ) -> Self::Output {
        let (map, _) = map_index_assignment.map.accept(self);
        let (value, _) = map_index_assignment.value.accept(self);

        if let Value::Record(record) = map {
            record
                .borrow_mut()
                .insert(map_index_assignment.key.lexeme.clone(), value.clone())
                .unwrap();

            return (value, false);
        }

        panic!("Cannot dot index into no record type");
    }

    fn visit_index_assignment(&mut self, index_assignment: &IndexAssignment) -> Self::Output {
        let (list, _) = index_assignment.list.accept(self);
        let (expression, _) = index_assignment.expression.accept(self);
        let (value, _) = index_assignment.value.accept(self);

        if let Value::List(list) = list {
            if let Value::Number(number) = expression {
                let number = (*number as i64) as usize;
                list.borrow_mut()[number] = value.clone();
                return (value, false);
            }
            panic!("Must use number to index into list type");
        }

        panic!("Must index into list type");
    }
}
