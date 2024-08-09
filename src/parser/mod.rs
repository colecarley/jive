/*
expression → assignment ;
assignment → identifier "=" assignment | equality ;
equality → comparison ( ( "!=" | "==" ) comparison )* ;
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term → factor ( ( "-" | "+" ) factor )* ;
factor → unary ( ( "/" | "*" ) unary )* ;
unary → ("!"|"-") unary
        | primary ;
primary → NUMBER | STRING | "true" | "false" | "nil"
        | "(" expression ")" ;
 */

use crate::token::{Token, TokenType};

pub mod accept;
pub mod expression;
pub mod statement;

use expression::{Assignment, Comparison, Equality, Expression, Factor, Primary, Term, Unary};
use statement::{DeclarationStatement, ExpressionStatement, PrintStatement, Statement};

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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        return statements;
    }

    fn statement(&mut self) -> Statement {
        if self.peek().token_type == TokenType::Print {
            return self.print_statement();
        } else if self.peek().token_type == TokenType::Make {
            return self.declaration_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!("Expected ';' after expression");
        }

        self.advance();

        return Statement::ExpressionStatement(ExpressionStatement { expression });
    }

    fn print_statement(&mut self) -> Statement {
        self.advance();
        let expression = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!("Expected ';' after expression");
        }

        self.advance();

        return Statement::PrintStatement(PrintStatement { expression });
    }

    fn declaration_statement(&mut self) -> Statement {
        self.advance();

        let identifier = self.advance();

        if self.peek().token_type == TokenType::Semicolon {
            self.advance();
            return Statement::DeclarationStatement(DeclarationStatement {
                identifier,
                expression: None,
            });
        }

        if self.peek().token_type != TokenType::Equal {
            panic!("Expected '=' after identifier");
        }

        self.advance();

        let expression = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!("Expected ';' after expression");
        }

        self.advance();

        return Statement::DeclarationStatement(DeclarationStatement {
            identifier,
            expression: Some(expression),
        });
    }

    fn expression(&mut self) -> Expression {
        return self.assignment();
    }

    fn assignment(&mut self) -> Expression {
        let expression = self.equality();

        if self.peek().token_type == TokenType::Equal {
            self.advance();
            let value = self.assignment();

            if let Expression::Primary(primary) = expression {
                if primary.value.token_type == TokenType::Identifier {
                    return Expression::Assignment(Box::new(Assignment {
                        identifier: primary.value,
                        value,
                    }));
                }
            }

            panic!("Invalid assignment target");
        }

        return expression;
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
            TokenType::Number
            | TokenType::String
            | TokenType::Boolean
            | TokenType::Nil
            | TokenType::Identifier => {
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

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
}
