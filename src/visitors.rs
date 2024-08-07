pub mod visitors {
    use crate::{
        parser::parser::{Accept, Comparison, Equality, Factor, Primary, Term, Unary},
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
    }

    pub struct AstPrinter {}

    impl AstPrinter {
        pub fn new() -> Self {
            AstPrinter {}
        }
    }

    impl Visitor for AstPrinter {
        type Output = String;

        fn visit_equality(&self, equality: &Equality) -> Self::Output {
            format!(
                "({} == {})",
                equality.left.accept(AstPrinter::new()),
                equality.right.accept(AstPrinter::new())
            )
        }

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output {
            format!(
                "({} {} {})",
                comparison.left.accept(self::AstPrinter::new()),
                comparison.operator.lexeme,
                comparison.right.accept(self::AstPrinter::new())
            )
        }

        fn visit_term(&self, term: &Term) -> Self::Output {
            format!(
                "({} {} {})",
                term.left.accept(self::AstPrinter::new()),
                term.operator.lexeme,
                term.right.accept(self::AstPrinter::new())
            )
        }

        fn visit_factor(&self, factor: &Factor) -> Self::Output {
            format!(
                "({} {} {})",
                factor.left.accept(self::AstPrinter::new()),
                factor.operator.lexeme,
                factor.right.accept(self::AstPrinter::new())
            )
        }

        fn visit_unary(&self, unary: &Unary) -> Self::Output {
            format!(
                "({}{})",
                unary.operator.lexeme,
                unary.right.accept(self::AstPrinter::new())
            )
        }

        fn visit_primary(&self, primary: &Primary) -> Self::Output {
            primary.value.lexeme.clone()
        }
    }

    pub struct TypeChecker {}

    impl TypeChecker {
        pub fn new() -> Self {
            TypeChecker {}
        }
    }

    impl Visitor for TypeChecker {
        type Output = TokenType;

        fn visit_equality(&self, equality: &Equality) -> Self::Output {
            let left_type = equality.left.accept(self::TypeChecker::new());
            let right_type = equality.right.accept(self::TypeChecker::new());

            if left_type != right_type {
                panic!("Operands must be of the same type");
            }

            return TokenType::Boolean;
        }

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output {
            let left_type = comparison.left.accept(self::TypeChecker::new());
            let right_type = comparison.right.accept(self::TypeChecker::new());

            if left_type != TokenType::Number || right_type != TokenType::Number {
                panic!("Operands must be numbers");
            }

            return TokenType::Boolean;
        }

        fn visit_term(&self, term: &Term) -> Self::Output {
            let left_type = term.left.accept(self::TypeChecker::new());
            let right_type = term.right.accept(self::TypeChecker::new());

            if left_type != TokenType::Number || right_type != TokenType::Number {
                panic!("Operands must be numbers");
            }

            TokenType::Number
        }

        fn visit_factor(&self, factor: &Factor) -> Self::Output {
            let left_type = factor.left.accept(self::TypeChecker::new());
            let right_type = factor.right.accept(self::TypeChecker::new());

            if left_type != TokenType::Number || right_type != TokenType::Number {
                panic!("Operands must be numbers");
            }

            TokenType::Number
        }

        fn visit_unary(&self, unary: &Unary) -> Self::Output {
            let right_type = unary.right.accept(self::TypeChecker::new());
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
    }

    pub struct Interpreter {}

    impl Interpreter {
        pub fn new() -> Self {
            Interpreter {}
        }
    }

    impl Visitor for Interpreter {
        type Output = f64;

        fn visit_equality(&self, equality: &Equality) -> Self::Output {
            match equality.operator.token_type {
                TokenType::EqualEqual => {
                    if equality.left.accept(self::Interpreter::new())
                        == equality.right.accept(self::Interpreter::new())
                    {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                TokenType::BangEqual => {
                    if equality.left.accept(self::Interpreter::new())
                        != equality.right.accept(self::Interpreter::new())
                    {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_comparison(&self, comparison: &Comparison) -> Self::Output {
            match comparison.operator.token_type {
                TokenType::Greater => {
                    if comparison.left.accept(self::Interpreter::new())
                        > comparison.right.accept(self::Interpreter::new())
                    {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                TokenType::GreaterEqual => {
                    if comparison.left.accept(self::Interpreter::new())
                        >= comparison.right.accept(self::Interpreter::new())
                    {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                TokenType::Less => {
                    if comparison.left.accept(self::Interpreter::new())
                        < comparison.right.accept(self::Interpreter::new())
                    {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                TokenType::LessEqual => {
                    if comparison.left.accept(self::Interpreter::new())
                        <= comparison.right.accept(self::Interpreter::new())
                    {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_term(&self, term: &Term) -> Self::Output {
            match term.operator.token_type {
                TokenType::Plus => {
                    term.left.accept(self::Interpreter::new())
                        + term.right.accept(self::Interpreter::new())
                }
                TokenType::Minus => {
                    term.left.accept(self::Interpreter::new())
                        - term.right.accept(self::Interpreter::new())
                }
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_factor(&self, factor: &Factor) -> Self::Output {
            match factor.operator.token_type {
                TokenType::Star => {
                    factor.left.accept(self::Interpreter::new())
                        * factor.right.accept(self::Interpreter::new())
                }
                TokenType::Slash => factor.left.accept(self::Interpreter::new()),
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_unary(&self, unary: &Unary) -> Self::Output {
            match unary.operator.token_type {
                TokenType::Minus => -unary.right.accept(self::Interpreter::new()),
                TokenType::Bang => {
                    if unary.right.accept(self::Interpreter::new()) == 0.0 {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                _ => panic!("Unexpected token type"),
            }
        }

        fn visit_primary(&self, primary: &Primary) -> Self::Output {
            match primary.value.token_type {
                TokenType::Number => primary.value.lexeme.parse().unwrap(),
                TokenType::Boolean => {
                    if primary.value.lexeme == "true" {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                TokenType::Nil => 0.0,
                _ => panic!("Unexpected token type"),
            }
        }
    }
}
