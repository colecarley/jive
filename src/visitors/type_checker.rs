use crate::{
    parser::{
        accept::Accept,
        expression::{
            Assignment, Call, Comparison, Equality, Factor, IfExpression, Primary, Term, Unary,
        },
        statement::{
            Block, DeclarationStatement, ExpressionStatement, IfStatement, PrintStatement,
            Statement, WhileStatement,
        },
    },
    token::TokenType,
};

use super::environment::Environment;

pub struct TypeChecker {
    environment: Environment<TokenType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut environment = Environment::<TokenType>::new();
        environment.declare_global("clock".to_string(), TokenType::Identifier);
        environment.declare_global("println".to_string(), TokenType::Identifier);
        environment.declare_global("input".to_string(), TokenType::Identifier);

        TypeChecker { environment }
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
                Statement::IfStatement(if_statement) => {
                    if_statement.accept(self);
                }
                Statement::WhileStatement(while_statement) => {
                    while_statement.accept(self);
                }
            }
        }
    }
}

impl super::Visitor for TypeChecker {
    type Output = TokenType;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
        let value_type = assignment.value.accept(self);
        self.environment
            .assign(assignment.identifier.lexeme.clone(), value_type.clone());

        value_type
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
        match primary.value.token_type {
            TokenType::Number => TokenType::Number,
            TokenType::Boolean => TokenType::Boolean,
            TokenType::String => TokenType::String,
            TokenType::Nil => TokenType::Nil,
            TokenType::Identifier => self.environment.get(primary.value.lexeme.clone()).clone(),
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Self::Output {
        expression_statement.expression.accept(self);

        TokenType::Nil
    }

    fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output {
        print_statement.expression.accept(self);

        TokenType::Nil
    }

    fn visit_declaration_statement(
        &mut self,
        declaration_statement: &DeclarationStatement,
    ) -> Self::Output {
        if let Some(expression) = &declaration_statement.expression {
            let value_type = expression.accept(self);
            self.environment
                .declare(declaration_statement.identifier.lexeme.clone(), value_type);

            return TokenType::Nil;
        }

        self.environment.declare(
            declaration_statement.identifier.lexeme.clone(),
            TokenType::Nil,
        );

        TokenType::Nil
    }

    fn visit_block(&mut self, block: &Block) -> Self::Output {
        let mut new_environment = Environment::new();
        new_environment.enclose(&Box::new(self.environment.clone()));
        self.environment = new_environment.clone();

        for statement in &block.statements {
            statement.accept(self);
        }

        self.environment = *self.environment.get_enclosing();

        TokenType::Nil
    }

    fn visit_if_statement(&mut self, if_statemnet: &IfStatement) -> Self::Output {
        let condition_type = if_statemnet.condition.accept(self);
        if condition_type != TokenType::Boolean {
            panic!("Condition must be a boolean");
        }

        let then_branch_type = if_statemnet.then_branch.accept(self);

        if then_branch_type != TokenType::Nil {
            panic!(
                "Then branch must not return a value, but got {:?}",
                then_branch_type
            );
        }

        if let Some(else_branch) = &if_statemnet.else_branch {
            let else_branch_type = else_branch.accept(self);

            if else_branch_type != TokenType::Nil {
                panic!(
                    "Else branch must not return a value, but got {:?}",
                    else_branch_type
                );
            }
        }

        TokenType::Nil
    }

    fn visit_if_expression(&mut self, if_expression: &IfExpression) -> Self::Output {
        let condition_type = if_expression.condition.accept(self);
        if condition_type != TokenType::Boolean {
            panic!("Condition must be a boolean");
        }

        // it doesn't matter if then_branch and else_branch are of the same type, but check the sub-expressions
        if_expression.then_branch.accept(self);
        if_expression.else_branch.accept(self);

        TokenType::Nil
    }

    fn visit_and(&mut self, and: &crate::parser::expression::And) -> Self::Output {
        let left_type = and.left.accept(self);
        let right_type = and.right.accept(self);

        if left_type != TokenType::Boolean || right_type != TokenType::Boolean {
            panic!("Operands must be booleans");
        }

        TokenType::Boolean
    }

    fn visit_or(&mut self, or: &crate::parser::expression::Or) -> Self::Output {
        let left_type = or.left.accept(self);
        let right_type = or.right.accept(self);

        if left_type != TokenType::Boolean || right_type != TokenType::Boolean {
            panic!("Operands must be booleans");
        }

        TokenType::Boolean
    }

    fn visit_while_statement(&mut self, while_statement: &WhileStatement) -> Self::Output {
        let condition_type = while_statement.condition.accept(self);
        if condition_type != TokenType::Boolean {
            panic!("Condition must be a boolean");
        }

        let body_type = while_statement.body.accept(self);

        if body_type != TokenType::Nil {
            panic!("Body must not return a value, but got {:?}", body_type);
        }

        TokenType::Nil
    }

    fn visit_call(&mut self, call: &Call) -> Self::Output {
        let callee_type = call.identifier.accept(self);
        // TODO: figure out a way to determine the type of the callee
        if callee_type != TokenType::Identifier {
            panic!("Callee must be a function");
        }

        for argument in &call.arguments {
            argument.accept(self);
        }

        // we don't know what the type of the function is, so just return nil
        TokenType::Nil
    }
}
