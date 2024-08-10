use std::{cell::RefCell, rc::Rc};

use crate::{
    parser::{
        accept::Accept,
        expression::{
            Assignment, Call, Comparison, Equality, Factor, IfExpression, Primary, Term, Unary,
        },
        statement::{
            Block, ExpressionStatement, FunctionDeclaration, IfStatement, PrintStatement, Return,
            Statement, VariableDeclaration, WhileStatement,
        },
    },
    token::TokenType,
};

use super::environment::Environment;

pub mod types;

use types::Type;

pub struct TypeChecker {
    environment: Rc<RefCell<Environment<Type>>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let environment = Rc::new(RefCell::new(Environment::<Type>::new()));

        environment
            .borrow_mut()
            .declare_global("clock".to_string(), Type::Function);

        environment
            .borrow_mut()
            .declare_global("println".to_string(), Type::Function);

        environment
            .borrow_mut()
            .declare_global("input".to_string(), Type::Function);

        TypeChecker { environment }
    }

    pub fn check(&mut self, statements: &Vec<Statement>) {
        for statement in statements {
            statement.accept(self);
        }
    }
}

impl super::Visitor for TypeChecker {
    type Output = Type;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
        let value_type = assignment.value.accept(self);
        self.environment
            .borrow_mut()
            .assign(assignment.identifier.lexeme.clone(), value_type.clone());

        value_type
    }

    fn visit_equality(&mut self, equality: &Equality) -> Self::Output {
        let left_type = equality.left.accept(self);
        let right_type = equality.right.accept(self);

        if left_type != right_type {
            panic!("Operands must be of the same type");
        }

        return Type::Boolean;
    }

    fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output {
        let left_type = comparison.left.accept(self);
        let right_type = comparison.right.accept(self);

        if left_type == Type::Unknown || right_type == Type::Unknown {
            return Type::Boolean;
        }

        if left_type != Type::Number || right_type != Type::Number {
            panic!("Operands must be numbers");
        }

        return Type::Boolean;
    }

    fn visit_term(&mut self, term: &Term) -> Self::Output {
        let left_type = term.left.accept(self);
        let right_type = term.right.accept(self);

        if left_type == Type::Unknown || right_type == Type::Unknown {
            return Type::Unknown;
        }

        if left_type != Type::Number || right_type != Type::Number {
            panic!("Operands must be numbers, line: {}", term.operator.line);
        }

        Type::Number
    }

    fn visit_factor(&mut self, factor: &Factor) -> Self::Output {
        let left_type = factor.left.accept(self);
        let right_type = factor.right.accept(self);

        if left_type == Type::Unknown || right_type == Type::Unknown {
            return Type::Unknown;
        }

        if left_type != Type::Number || right_type != Type::Number {
            panic!("Operands must be numbers");
        }

        Type::Number
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        let right_type = unary.right.accept(self);
        if unary.operator.token_type == TokenType::Minus {
            if right_type != Type::Number {
                panic!("Unary operator - can only be applied to numbers");
            } else {
                return Type::Number;
            }
        }

        if unary.operator.token_type == TokenType::Bang {
            if right_type != Type::Boolean {
                panic!("Unary operator ! can only be applied to booleans");
            } else {
                return Type::Boolean;
            }
        }

        panic!("Unknown unary operator");
    }

    fn visit_primary(&mut self, primary: &Primary) -> Self::Output {
        match primary.value.token_type {
            TokenType::Number => Type::Number,
            TokenType::Boolean => Type::Boolean,
            TokenType::String => Type::String,
            TokenType::Nil => Type::Nil,
            TokenType::Identifier => self
                .environment
                .borrow_mut()
                .get(primary.value.lexeme.clone()),
            _ => panic!("Unexpected token type"),
        }
    }

    fn visit_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Self::Output {
        expression_statement.expression.accept(self);

        Type::Nil
    }

    fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output {
        print_statement.expression.accept(self);

        Type::Nil
    }

    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Self::Output {
        if let Some(expression) = &variable_declaration.expression {
            let value_type = expression.accept(self);
            self.environment
                .borrow_mut()
                .declare(variable_declaration.identifier.lexeme.clone(), value_type);

            return Type::Nil;
        }

        self.environment
            .borrow_mut()
            .declare(variable_declaration.identifier.lexeme.clone(), Type::Nil);

        Type::Nil
    }

    fn visit_block(&mut self, block: &Block) -> Self::Output {
        let new_environment = Rc::new(RefCell::new(Environment::new()));
        new_environment
            .borrow_mut()
            .enclose(self.environment.clone());
        self.environment = new_environment.clone();

        for statement in &block.statements {
            statement.accept(self);
        }

        self.environment = new_environment.borrow_mut().get_enclosing();

        Type::Nil
    }

    fn visit_if_statement(&mut self, if_statemnet: &IfStatement) -> Self::Output {
        let condition_type = if_statemnet.condition.accept(self);
        if condition_type != Type::Boolean || condition_type == Type::Unknown {
            panic!("Condition must be a boolean");
        }

        let then_branch_type = if_statemnet.then_branch.accept(self);

        if then_branch_type != Type::Nil {
            panic!(
                "Then branch must not return a value, but got {:?}",
                then_branch_type
            );
        }

        if let Some(else_branch) = &if_statemnet.else_branch {
            let else_branch_type = else_branch.accept(self);

            if else_branch_type != Type::Nil {
                panic!(
                    "Else branch must not return a value, but got {:?}",
                    else_branch_type
                );
            }
        }

        Type::Nil
    }

    fn visit_if_expression(&mut self, if_expression: &IfExpression) -> Self::Output {
        let condition_type = if_expression.condition.accept(self);

        if condition_type != Type::Boolean || condition_type == Type::Unknown {
            panic!("Condition must be a boolean");
        }

        if_expression.then_branch.accept(self);
        if_expression.else_branch.accept(self);

        Type::Unknown
    }

    fn visit_and(&mut self, and: &crate::parser::expression::And) -> Self::Output {
        let left_type = and.left.accept(self);
        let right_type = and.right.accept(self);

        if (left_type != Type::Boolean || left_type == Type::Unknown)
            || (right_type != Type::Boolean || left_type == Type::Unknown)
        {
            panic!("Operands must be booleans");
        }

        Type::Boolean
    }

    fn visit_or(&mut self, or: &crate::parser::expression::Or) -> Self::Output {
        let left_type = or.left.accept(self);
        let right_type = or.right.accept(self);

        if (left_type != Type::Boolean || left_type == Type::Unknown)
            || (right_type != Type::Boolean || right_type == Type::Unknown)
        {
            panic!("Operands must be booleans");
        }

        Type::Boolean
    }

    fn visit_while_statement(&mut self, while_statement: &WhileStatement) -> Self::Output {
        let condition_type = while_statement.condition.accept(self);
        if condition_type != Type::Boolean || condition_type == Type::Unknown {
            panic!("Condition must be a boolean");
        }

        let body_type = while_statement.body.accept(self);

        if body_type != Type::Nil {
            panic!("Body must not return a value, but got {:?}", body_type);
        }

        Type::Nil
    }

    fn visit_call(&mut self, call: &Call) -> Self::Output {
        let callee_type = call.identifier.accept(self);
        if callee_type != Type::Function && callee_type != Type::Unknown {
            panic!("Callee must be a function, but got {:?}", callee_type);
        }

        for argument in &call.arguments {
            argument.accept(self);
        }

        // TODO: figure out the return type of the function

        Type::Unknown
    }

    fn visit_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Self::Output {
        self.environment.borrow_mut().declare(
            function_declaration.identifier.lexeme.clone(),
            Type::Function,
        );

        let new_environment = Rc::new(RefCell::new(Environment::new()));
        new_environment
            .borrow_mut()
            .enclose(self.environment.clone());
        self.environment = new_environment.clone();

        for parameter in &function_declaration.parameters {
            self.environment
                .borrow_mut()
                .declare(parameter.lexeme.clone(), Type::Unknown);
        }

        function_declaration.body.accept(self);

        self.environment = new_environment.borrow_mut().get_enclosing();

        Type::Nil
    }

    fn visit_return(&mut self, return_statement: &Return) -> Self::Output {
        match &return_statement.value {
            Some(value) => value.accept(self),
            None => Type::Nil,
        }
    }
}
