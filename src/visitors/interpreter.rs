use std::collections::HashMap;

use crate::{
    parser::{
        Accept, Assignment, Comparison, DeclarationStatement, Equality, ExpressionStatement,
        Factor, Primary, PrintStatement, Statement, Term, Unary,
    },
    token::TokenType,
};

pub struct Interpreter {
    pub globals: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, statements: &Vec<Statement>) -> Value {
        for statement in statements {
            match statement {
                Statement::ExpressionStatement(expression_statement) => {
                    expression_statement.accept(self);
                }
                Statement::PrintStatement(print_statement) => {
                    print_statement.accept(self);
                }
                Statement::DeclarationStatement(declaration_statement) => {
                    declaration_statement.accept(self);
                }
            }
        }

        Value::Nil
    }
}
impl super::Visitor for Interpreter {
    type Output = Value;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
        let value = assignment.value.accept(self);
        if self.globals.contains_key(&assignment.identifier.lexeme) {
            self.globals
                .insert(assignment.identifier.lexeme.clone(), value.clone());
        } else {
            panic!("Undefined variable {}", assignment.identifier.lexeme);
        }

        Value::Nil
    }

    fn visit_equality(&mut self, equality: &Equality) -> Self::Output {
        match equality.operator.token_type {
            TokenType::EqualEqual => {
                Value::Boolean(equality.left.accept(self) == equality.right.accept(self))
            }
            TokenType::BangEqual => {
                Value::Boolean(equality.left.accept(self) != equality.right.accept(self))
            }
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output {
        match comparison.operator.token_type {
            TokenType::Greater => {
                Value::Boolean(comparison.left.accept(self) > comparison.right.accept(self))
            }
            TokenType::GreaterEqual => {
                Value::Boolean(comparison.left.accept(self) >= comparison.right.accept(self))
            }
            TokenType::Less => {
                Value::Boolean(comparison.left.accept(self) < comparison.right.accept(self))
            }
            TokenType::LessEqual => {
                Value::Boolean(comparison.left.accept(self) <= comparison.right.accept(self))
            }
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_term(&mut self, term: &Term) -> Self::Output {
        match term.operator.token_type {
            TokenType::Plus => term.left.accept(self) + term.right.accept(self),
            TokenType::Minus => term.left.accept(self) - term.right.accept(self),
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_factor(&mut self, factor: &Factor) -> Self::Output {
        match factor.operator.token_type {
            TokenType::Star => factor.left.accept(self) * factor.right.accept(self),
            TokenType::Slash => factor.left.accept(self),
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        match unary.operator.token_type {
            TokenType::Minus => -unary.right.accept(self),
            TokenType::Bang => Value::Boolean(unary.right.accept(self) == Value::Boolean(false)),
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_primary(&mut self, primary: &Primary) -> Self::Output {
        match primary.value.token_type {
            TokenType::Number => Value::Number(primary.value.lexeme.parse().unwrap()),
            TokenType::Boolean => Value::Boolean(primary.value.lexeme == "true"),
            TokenType::String => Value::String(primary.value.lexeme.clone()),
            TokenType::Identifier => {
                if self.globals.contains_key(&primary.value.lexeme) {
                    self.globals.get(&primary.value.lexeme).unwrap().clone()
                } else {
                    panic!("Undefined variable {}", primary.value.lexeme)
                }
            }
            TokenType::Nil => Value::Nil,
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Self::Output {
        expression_statement.expression.accept(self)
    }

    fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output {
        let value = print_statement.expression.accept(self);

        match value {
            Value::Number(number) => println!("{}", number),
            Value::Boolean(boolean) => println!("{}", boolean),
            Value::String(string) => println!("{}", string),
            Value::Nil => println!("nil"),
        }

        Value::Nil
    }

    fn visit_declaration_statement(
        &mut self,
        declaration_statement: &DeclarationStatement,
    ) -> Self::Output {
        if let Some(expression) = &declaration_statement.expression {
            let value = expression.accept(self);
            self.globals
                .insert(declaration_statement.identifier.lexeme.clone(), value);

            return Value::Nil;
        }
        self.globals
            .insert(declaration_statement.identifier.lexeme.clone(), Value::Nil);

        Value::Nil
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
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

impl std::ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Value::Boolean(right) => Value::Boolean(!right),
            _ => panic!("Unary operator ! can only be applied to booleans"),
        }
    }
}
