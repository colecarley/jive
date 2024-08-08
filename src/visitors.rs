pub mod visitors {
    use crate::{
        parser::parser::{
            Accept, Comparison, Equality, ExpressionStatement, Factor, Primary, PrintStatement,
            Statement, Term, Unary,
        },
        token::token::TokenType,
    };

    pub trait Visitor {
        type Output;

        fn visit_equality(&self, equality: &Equality) -> Self::Output;

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output;

        fn visit_term(&self, term: &Term) -> Self::Output;

        fn visit_factor(&self, factor: &Factor) -> Self::Output;

        fn visit_unary(&self, unary: &Unary) -> Self::Output;

        fn visit_primary(&self, primary: &Primary) -> Self::Output;

        fn visit_expression_statement(
            &self,
            expression_statement: &ExpressionStatement,
        ) -> Self::Output;

        fn visit_print_statement(&self, print_statement: &PrintStatement) -> Self::Output;
    }

    pub struct AstPrinter {}

    impl AstPrinter {
        pub fn new() -> Self {
            AstPrinter {}
        }

        pub fn print(&self, statements: &Vec<Statement>) {
            let mut result = String::new();
            for statement in statements {
                match statement {
                    Statement::ExpressionStatement(expression_statement) => {
                        result.push_str(&expression_statement.accept(self));
                    }
                    Statement::PrintStatement(print_statement) => {
                        result.push_str(&print_statement.accept(self));
                    }
                }
                result.push('\n');
            }

            println!("{}", result)
        }
    }

    impl Visitor for AstPrinter {
        type Output = String;

        fn visit_equality(&self, equality: &Equality) -> Self::Output {
            format!(
                "({} == {})",
                equality.left.accept(self),
                equality.right.accept(self)
            )
        }

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output {
            format!(
                "({} {} {})",
                comparison.left.accept(self),
                comparison.operator.lexeme,
                comparison.right.accept(self)
            )
        }

        fn visit_term(&self, term: &Term) -> Self::Output {
            format!(
                "({} {} {})",
                term.left.accept(self),
                term.operator.lexeme,
                term.right.accept(self)
            )
        }

        fn visit_factor(&self, factor: &Factor) -> Self::Output {
            format!(
                "({} {} {})",
                factor.left.accept(self),
                factor.operator.lexeme,
                factor.right.accept(self)
            )
        }

        fn visit_unary(&self, unary: &Unary) -> Self::Output {
            format!("({}{})", unary.operator.lexeme, unary.right.accept(self))
        }

        fn visit_primary(&self, primary: &Primary) -> Self::Output {
            primary.value.lexeme.clone()
        }

        fn visit_expression_statement(
            &self,
            expression_statement: &ExpressionStatement,
        ) -> Self::Output {
            expression_statement.expression.accept(self)
        }

        fn visit_print_statement(&self, print_statement: &PrintStatement) -> Self::Output {
            format!("print ({})", print_statement.expression.accept(self))
        }
    }

    pub struct TypeChecker {}

    impl TypeChecker {
        pub fn new() -> Self {
            TypeChecker {}
        }

        pub fn check(&self, statements: &Vec<Statement>) {
            for statement in statements {
                match statement {
                    Statement::ExpressionStatement(expression_statement) => {
                        expression_statement.accept(self);
                    }
                    Statement::PrintStatement(print_statement) => {
                        print_statement.accept(self);
                    }
                }
            }
        }
    }

    impl Visitor for TypeChecker {
        type Output = TokenType;

        fn visit_equality(&self, equality: &Equality) -> Self::Output {
            let left_type = equality.left.accept(self);
            let right_type = equality.right.accept(self);

            if left_type != right_type {
                panic!("Operands must be of the same type");
            }

            return TokenType::Boolean;
        }

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output {
            let left_type = comparison.left.accept(self);
            let right_type = comparison.right.accept(self);

            if left_type != TokenType::Number || right_type != TokenType::Number {
                panic!("Operands must be numbers");
            }

            return TokenType::Boolean;
        }

        fn visit_term(&self, term: &Term) -> Self::Output {
            let left_type = term.left.accept(self);
            let right_type = term.right.accept(self);

            if left_type != TokenType::Number || right_type != TokenType::Number {
                panic!("Operands must be numbers");
            }

            TokenType::Number
        }

        fn visit_factor(&self, factor: &Factor) -> Self::Output {
            let left_type = factor.left.accept(self);
            let right_type = factor.right.accept(self);

            if left_type != TokenType::Number || right_type != TokenType::Number {
                panic!("Operands must be numbers");
            }

            TokenType::Number
        }

        fn visit_unary(&self, unary: &Unary) -> Self::Output {
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

        fn visit_primary(&self, primary: &Primary) -> Self::Output {
            primary.value.token_type.clone()
        }

        fn visit_expression_statement(
            &self,
            expression_statement: &ExpressionStatement,
        ) -> Self::Output {
            expression_statement.expression.accept(self)
        }

        fn visit_print_statement(&self, print_statement: &PrintStatement) -> Self::Output {
            print_statement.expression.accept(self)
        }
    }

    pub struct Interpreter {}

    impl Interpreter {
        pub fn new() -> Self {
            Interpreter {}
        }

        pub fn evaluate(&self, statements: &Vec<Statement>) -> Value {
            for statement in statements {
                match statement {
                    Statement::ExpressionStatement(expression_statement) => {
                        expression_statement.accept(self);
                    }
                    Statement::PrintStatement(print_statement) => {
                        print_statement.accept(self);
                    }
                }
            }

            Value::Nil
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
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

        fn visit_equality(&self, equality: &Equality) -> Self::Output {
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

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output {
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

        fn visit_term(&self, term: &Term) -> Self::Output {
            match term.operator.token_type {
                TokenType::Plus => term.left.accept(self) + term.right.accept(self),
                TokenType::Minus => term.left.accept(self) - term.right.accept(self),
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_factor(&self, factor: &Factor) -> Self::Output {
            match factor.operator.token_type {
                TokenType::Star => factor.left.accept(self) * factor.right.accept(self),
                TokenType::Slash => factor.left.accept(self),
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_unary(&self, unary: &Unary) -> Self::Output {
            match unary.operator.token_type {
                TokenType::Minus => -unary.right.accept(self),
                TokenType::Bang => {
                    Value::Boolean(unary.right.accept(self) == Value::Boolean(false))
                }
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_primary(&self, primary: &Primary) -> Self::Output {
            match primary.value.token_type {
                TokenType::Number => Value::Number(primary.value.lexeme.parse().unwrap()),
                TokenType::Boolean => Value::Boolean(primary.value.lexeme == "true"),
                TokenType::String => Value::String(primary.value.lexeme.clone()),
                TokenType::Nil => Value::Nil,
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_expression_statement(
            &self,
            expression_statement: &ExpressionStatement,
        ) -> Self::Output {
            expression_statement.expression.accept(self)
        }

        fn visit_print_statement(&self, print_statement: &PrintStatement) -> Self::Output {
            let value = print_statement.expression.accept(self);

            match value {
                Value::Number(number) => println!("{}", number),
                Value::Boolean(boolean) => println!("{}", boolean),
                Value::String(string) => println!("{}", string),
                Value::Nil => println!("nil"),
                _ => panic!("Unexpected value type, {:?}", value),
            }

            Value::Nil
        }
    }
}
