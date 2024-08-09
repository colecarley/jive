use crate::{
    parser::{
        accept::Accept,
        expression::{Assignment, Comparison, Equality, Factor, Primary, Term, Unary},
        statement::{DeclarationStatement, ExpressionStatement, PrintStatement, Statement},
    },
    token::TokenType,
};

pub mod environment;
pub mod value;

use environment::Environment;
use value::Value;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
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
        self.environment
            .insert(assignment.identifier.lexeme.clone(), value);

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
            TokenType::Identifier => self.environment.get(primary.value.lexeme.clone()),
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
            self.environment
                .insert(declaration_statement.identifier.lexeme.clone(), value);

            return Value::Nil;
        }
        self.environment
            .insert(declaration_statement.identifier.lexeme.clone(), Value::Nil);

        Value::Nil
    }
}
