/*
expression → equality ;
equality → comparison ( ( "!=" | "==" ) comparison )* ;
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term → factor ( ( "-" | "+" ) factor )* ;
factor → unary ( ( "/" | "*" ) unary )* ;
unary → ("!"|"-") unary
        | primary ;
primary → NUMBER | STRING | "true" | "false" | "nil"
        | "(" expression ")" ;
 */

pub mod parser {
    use crate::{
        token::token::{Token, TokenType},
        visitors::visitors::Visitor,
    };
    pub struct Parser {
        tokens: Vec<Token>,
        position: usize,
    }

    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Parser {
            Parser {
                tokens,
                position: 0,
            }
        }

        pub fn parse(&mut self) -> Expression {
            return self.expression();
        }

        fn expression(&mut self) -> Expression {
            return self.equality();
        }

        fn equality(&mut self) -> Expression {
            let mut first = self.comparison();
            while match self.peek().token_type {
                TokenType::BangEqual | TokenType::EqualEqual => true,
                _ => false,
            } {
                let operator = self.advance();
                let second = self.comparison();
                first = Expression::Equality(Box::new(Equality::new(first, operator, second)));
            }

            first
        }

        fn comparison(&mut self) -> Expression {
            let mut first = self.term();

            while match self.peek().token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => true,
                _ => false,
            } {
                let operator = self.advance();
                let second = self.term();
                first = Expression::Comparison(Box::new(Comparison::new(first, operator, second)));
            }

            first
        }

        fn term(&mut self) -> Expression {
            let mut first = self.factor();

            while match self.peek().token_type {
                TokenType::Minus | TokenType::Plus => true,
                _ => false,
            } {
                let operator = self.advance();
                let second = self.factor();
                first = Expression::Term(Box::new(Term::new(first, operator, second)));
            }

            first
        }

        fn factor(&mut self) -> Expression {
            let mut first = self.unary();

            while match self.peek().token_type {
                TokenType::Slash | TokenType::Star => true,
                _ => false,
            } {
                let operator = self.advance();
                let second = self.unary();
                first = Expression::Factor(Box::new(Factor::new(first, operator, second)));
            }

            first
        }

        fn unary(&mut self) -> Expression {
            if match self.peek().token_type {
                TokenType::Bang | TokenType::Minus => true,
                _ => false,
            } {
                let operator = self.peek_previous();
                let right = self.unary();
                return Expression::Unary(Box::new(Unary::new(operator, right)));
            } else {
                return self.primary();
            }
        }

        fn primary(&mut self) -> Expression {
            match self.peek().token_type {
                TokenType::Number | TokenType::String | TokenType::Boolean | TokenType::Nil => {
                    let value = self.advance();
                    return Expression::Primary(Box::new(Primary::new(value)));
                }
                TokenType::LParen => {
                    self.advance();
                    let expression = self.expression();
                    if self.peek().token_type != TokenType::RParen {
                        panic!("Expected ')' after expression");
                    }
                    self.advance();
                    return expression;
                }
                _ => panic!("Unexpected token, {:?}", self.peek().token_type),
            }
        }

        fn advance(&mut self) -> Token {
            self.position += 1;
            self.tokens[self.position - 1].clone()
        }

        fn peek_previous(&self) -> Token {
            self.tokens[self.position - 1].clone()
        }

        fn peek(&self) -> Token {
            self.tokens[self.position].clone()
        }
    }

    pub struct Equality {
        pub left: Expression,
        pub operator: Token,
        pub right: Expression,
    }

    impl Equality {
        fn new(left: Expression, operator: Token, right: Expression) -> Equality {
            Equality {
                left,
                operator,
                right,
            }
        }
    }

    pub struct Comparison {
        pub left: Expression,
        pub operator: Token,
        pub right: Expression,
    }

    impl Comparison {
        fn new(left: Expression, operator: Token, right: Expression) -> Comparison {
            Comparison {
                left,
                operator,
                right,
            }
        }
    }

    pub struct Term {
        pub left: Expression,
        pub operator: Token,
        pub right: Expression,
    }

    impl Term {
        fn new(left: Expression, operator: Token, right: Expression) -> Term {
            Term {
                left,
                operator,
                right,
            }
        }
    }

    pub struct Factor {
        pub left: Expression,
        pub operator: Token,
        pub right: Expression,
    }

    impl Factor {
        fn new(left: Expression, operator: Token, right: Expression) -> Factor {
            Factor {
                left,
                operator,
                right,
            }
        }
    }

    pub struct Unary {
        pub operator: Token,
        pub right: Expression,
    }

    impl Unary {
        fn new(operator: Token, right: Expression) -> Unary {
            Unary { operator, right }
        }
    }

    pub struct Primary {
        pub value: Token,
    }

    impl Primary {
        fn new(value: Token) -> Primary {
            Primary { value }
        }
    }

    pub trait Accept {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output;
    }

    impl Accept for Equality {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            visitor.visit_equality(self)
        }
    }

    impl Accept for Comparison {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            visitor.visit_comparison(self)
        }
    }

    impl Accept for Term {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            visitor.visit_term(self)
        }
    }

    impl Accept for Factor {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            visitor.visit_factor(self)
        }
    }

    impl Accept for Unary {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            visitor.visit_unary(self)
        }
    }

    impl Accept for Primary {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            visitor.visit_primary(self)
        }
    }

    impl Accept for Expression {
        fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
            match self {
                Expression::Equality(equality) => equality.accept(visitor),
                Expression::Comparison(comparison) => comparison.accept(visitor),
                Expression::Term(term) => term.accept(visitor),
                Expression::Factor(factor) => factor.accept(visitor),
                Expression::Unary(unary) => unary.accept(visitor),
                Expression::Primary(primary) => primary.accept(visitor),
            }
        }
    }

    pub enum Expression {
        Equality(Box<Equality>),
        Comparison(Box<Comparison>),
        Term(Box<Term>),
        Factor(Box<Factor>),
        Unary(Box<Unary>),
        Primary(Box<Primary>),
    }
}
