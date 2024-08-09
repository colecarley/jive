use std::collections::HashMap;

use crate::{
    parser::{
        accept::Accept,
        expression::{Assignment, Comparison, Equality, Factor, Primary, Term, Unary},
        statement::{DeclarationStatement, ExpressionStatement, PrintStatement, Statement},
    },
    token::TokenType,
};

pub struct TypeChecker {
    pub globals: HashMap<String, TokenType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            globals: HashMap::new(),
        }
    }

    pub fn check(&mut self, statements: &Vec<Statement>) {
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
                Statement::Block(block) => {
                    block.accept(self);
                }
            }
        }
    }
}

impl super::Visitor for TypeChecker {
    type Output = TokenType;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
        let value_type = assignment.value.accept(self);
        self.globals
            .insert(assignment.identifier.lexeme.clone(), value_type);

        TokenType::Nil
    }

    fn visit_equality(&mut self, equality: &Equality) -> Self::Output {
        let left_type = equality.left.accept(self);
        let right_type = equality.right.accept(self);

        if left_type != right_type {
            panic!("Operands must be of the same type");
        }

        return TokenType::Boolean;
    }

    fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output {
        let left_type = comparison.left.accept(self);
        let right_type = comparison.right.accept(self);

        if left_type != TokenType::Number || right_type != TokenType::Number {
            panic!("Operands must be numbers");
        }

        return TokenType::Boolean;
    }

    fn visit_term(&mut self, term: &Term) -> Self::Output {
        let left_type = term.left.accept(self);
        let right_type = term.right.accept(self);

        if left_type != TokenType::Number || right_type != TokenType::Number {
            panic!("Operands must be numbers");
        }

        TokenType::Number
    }

    fn visit_factor(&mut self, factor: &Factor) -> Self::Output {
        let left_type = factor.left.accept(self);
        let right_type = factor.right.accept(self);

        if left_type != TokenType::Number || right_type != TokenType::Number {
            panic!("Operands must be numbers");
        }

        TokenType::Number
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        let right_type = unary.right.accept(self);
        if unary.operator.token_type == TokenType::Minus {
            if right_type != TokenType::Number {
                panic!("Unary operator - can only be applied to numbers");
            } else {
                return TokenType::Number;
            }
        }

        if unary.operator.token_type == TokenType::Bang {
            if right_type != TokenType::Boolean {
                panic!("Unary operator ! can only be applied to booleans");
            } else {
                return TokenType::Boolean;
            }
        }

        panic!("Unknown unary operator");
    }

    fn visit_primary(&mut self, primary: &Primary) -> Self::Output {
        // primary.value.token_type.clone()
        match primary.value.token_type {
            TokenType::Number => TokenType::Number,
            TokenType::Boolean => TokenType::Boolean,
            TokenType::String => TokenType::String,
            TokenType::Nil => TokenType::Nil,
            TokenType::Identifier => {
                if self.globals.contains_key(&primary.value.lexeme) {
                    self.globals.get(&primary.value.lexeme).unwrap().clone()
                } else {
                    panic!("Undefined variable {}", primary.value.lexeme)
                }
            }
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
        print_statement.expression.accept(self)
    }

    fn visit_declaration_statement(
        &mut self,
        declaration_statement: &DeclarationStatement,
    ) -> Self::Output {
        if let Some(expression) = &declaration_statement.expression {
            let value_type = expression.accept(self);
            self.globals
                .insert(declaration_statement.identifier.lexeme.clone(), value_type);

            return TokenType::Nil;
        }

        self.globals.insert(
            declaration_statement.identifier.lexeme.clone(),
            TokenType::Nil,
        );

        TokenType::Nil
    }

    fn visit_block(&mut self, block: &crate::parser::statement::Block) -> Self::Output {
        for statement in &block.statements {
            statement.accept(self);
        }

        TokenType::Nil
    }
}
