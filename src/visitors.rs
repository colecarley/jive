pub mod visitors {
    use std::collections::HashMap;

    use crate::{
        parser::parser::{
            Accept, Assignment, Comparison, DeclarationStatement, Equality, ExpressionStatement,
            Factor, Primary, PrintStatement, Statement, Term, Unary,
        },
        token::token::TokenType,
    };

    pub trait Visitor {
        type Output;

        fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output;

        fn visit_equality(&mut self, equality: &Equality) -> Self::Output;

        fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output;

        fn visit_term(&mut self, term: &Term) -> Self::Output;

        fn visit_factor(&mut self, factor: &Factor) -> Self::Output;

        fn visit_unary(&mut self, unary: &Unary) -> Self::Output;

        fn visit_primary(&mut self, primary: &Primary) -> Self::Output;

        fn visit_expression_statement(
            &mut self,
            expression_statement: &ExpressionStatement,
        ) -> Self::Output;

        fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output;

        fn visit_declaration_statement(
            &mut self,
            declarion_statement: &DeclarationStatement,
        ) -> Self::Output;
    }

    pub struct AstPrinter {}

    impl AstPrinter {
        pub fn new() -> Self {
            AstPrinter {}
        }

        pub fn print(&mut self, statements: &Vec<Statement>) {
            let mut result = String::new();
            for statement in statements {
                match statement {
                    Statement::ExpressionStatement(expression_statement) => {
                        result.push_str(&expression_statement.accept(self));
                    }
                    Statement::PrintStatement(print_statement) => {
                        result.push_str(&print_statement.accept(self));
                    }
                    Statement::DeclarationStatement(declaration_statement) => {
                        result.push_str(&declaration_statement.accept(self));
                    }
                }
                result.push('\n');
            }

            println!("{}", result)
        }
    }

    impl Visitor for AstPrinter {
        type Output = String;

        fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Output {
            format!(
                "{} = {}",
                assignment.identifier.lexeme,
                assignment.value.accept(self)
            )
        }

        fn visit_equality(&mut self, equality: &Equality) -> Self::Output {
            format!(
                "({} == {})",
                equality.left.accept(self),
                equality.right.accept(self)
            )
        }

        fn visit_comparison(&mut self, comparison: &Comparison) -> Self::Output {
            format!(
                "({} {} {})",
                comparison.left.accept(self),
                comparison.operator.lexeme,
                comparison.right.accept(self)
            )
        }

        fn visit_term(&mut self, term: &Term) -> Self::Output {
            format!(
                "({} {} {})",
                term.left.accept(self),
                term.operator.lexeme,
                term.right.accept(self)
            )
        }

        fn visit_factor(&mut self, factor: &Factor) -> Self::Output {
            format!(
                "({} {} {})",
                factor.left.accept(self),
                factor.operator.lexeme,
                factor.right.accept(self)
            )
        }

        fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
            format!("({}{})", unary.operator.lexeme, unary.right.accept(self))
        }

        fn visit_primary(&mut self, primary: &Primary) -> Self::Output {
            primary.value.lexeme.clone()
        }

        fn visit_expression_statement(
            &mut self,
            expression_statement: &ExpressionStatement,
        ) -> Self::Output {
            expression_statement.expression.accept(self)
        }

        fn visit_print_statement(&mut self, print_statement: &PrintStatement) -> Self::Output {
            format!("print ({})", print_statement.expression.accept(self))
        }

        fn visit_declaration_statement(
            &mut self,
            declarion_statement: &DeclarationStatement,
        ) -> Self::Output {
            let identifier = declarion_statement.identifier.lexeme.clone();
            if let Some(expression) = &declarion_statement.expression {
                format!("make {} = {}", identifier, expression.accept(self))
            } else {
                format!("make {}", identifier)
            }
        }
    }

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
                }
            }
        }
    }

    impl Visitor for TypeChecker {
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
    }

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

    impl Visitor for Interpreter {
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
                TokenType::Bang => {
                    Value::Boolean(unary.right.accept(self) == Value::Boolean(false))
                }
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
}
