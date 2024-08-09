/*
program → statement* EOF ;
statement → printStatement | declarationStatement | expressionStatement | ifStatement | block ;
ifStatement → "if"  expression  '{' statement '}' ( "else" '{' statement '}' )? ;
block → "{" statement* "}" ;
expression → assignment ;
assignment → identifier "=" assignment | ifExpression ;
ifExpression → expression "if" expression "else" expression | equality ;
or → and ( "||" and )* | and;
and → equality ( "&&" equality )* | equality;
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

use expression::{
    And, Assignment, Comparison, Equality, Expression, Factor, IfExpression, Or, Primary, Term,
    Unary,
};
use statement::{
    Block, DeclarationStatement, ExpressionStatement, IfStatement, PrintStatement, Statement,
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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        return statements;
    }

    fn statement(&mut self) -> Statement {
        match self.peek().token_type {
            TokenType::Print => return self.print_statement(),
            TokenType::Make => return self.declaration_statement(),
            TokenType::LBrace => return self.block(),
            TokenType::If => return self.if_statement(),
            _ => return self.expression_statement(),
        }
    }

    fn if_statement(&mut self) -> Statement {
        self.advance();

        let condition = self.expression();

        let then_branch = self.statement();

        let else_branch = if self.peek().token_type == TokenType::Else {
            self.advance();

            let else_branch = self.statement();

            Some(Box::new(else_branch))
        } else {
            None
        };

        return Statement::IfStatement(Box::new(IfStatement {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        }));
    }

    fn block(&mut self) -> Statement {
        self.advance();
        let mut statements = vec![];

        while self.peek().token_type != TokenType::RBrace && !self.is_at_end() {
            statements.push(self.statement());
        }

        if self.peek().token_type != TokenType::RBrace {
            panic!("Expected '}}' after block");
        }

        self.advance();

        return Statement::Block(Box::new(Block { statements }));
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!("Expected ';' after expression at line {}", self.peek().line);
        }

        self.advance();

        return Statement::ExpressionStatement(Box::new(ExpressionStatement { expression }));
    }

    fn print_statement(&mut self) -> Statement {
        self.advance();
        let expression = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!("Expected ';' after expression");
        }

        self.advance();

        return Statement::PrintStatement(Box::new(PrintStatement { expression }));
    }

    fn declaration_statement(&mut self) -> Statement {
        self.advance();

        let identifier = self.advance();

        if self.peek().token_type == TokenType::Semicolon {
            self.advance();
            return Statement::DeclarationStatement(Box::new(DeclarationStatement {
                identifier,
                expression: None,
            }));
        }

        if self.peek().token_type != TokenType::Equal {
            panic!("Expected '=' after identifier");
        }

        self.advance();

        let expression = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!(
                "Expected ';' after expression, found {:?}",
                self.peek().token_type
            );
        }

        self.advance();

        return Statement::DeclarationStatement(Box::new(DeclarationStatement {
            identifier,
            expression: Some(expression),
        }));
    }

    fn expression(&mut self) -> Expression {
        return self.assignment();
    }

    fn assignment(&mut self) -> Expression {
        let first = self.if_expression();

        while self.peek().token_type == TokenType::Equal {
            self.advance();
            let second = self.assignment();

            match first {
                Expression::Primary(primary) => {
                    return Expression::Assignment(Box::new(Assignment {
                        identifier: primary.value,
                        value: second,
                    }));
                }
                _ => {
                    panic!("Invalid assignment target");
                }
            }
        }

        first
    }

    fn if_expression(&mut self) -> Expression {
        let mut first = self.or();

        while self.peek().token_type == TokenType::If {
            self.advance();
            let condition = self.or();

            if self.peek().token_type != TokenType::Else {
                panic!("Expected 'else' after if expression");
            }

            self.advance();

            let else_expression = self.or();

            first = Expression::IfExpression(Box::new(IfExpression {
                condition: condition,
                then_branch: first,
                else_branch: else_expression,
            }));
        }

        first
    }

    fn or(&mut self) -> Expression {
        let mut first = self.and();

        while self.peek().token_type == TokenType::Or {
            self.advance();
            let right = self.and();
            first = Expression::Or(Box::new(Or { left: first, right }));
        }

        first
    }

    fn and(&mut self) -> Expression {
        let mut first = self.equality();

        while self.peek().token_type == TokenType::And {
            self.advance();
            let right = self.equality();
            first = Expression::And(Box::new(And { left: first, right }));
        }

        first
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
            let operator = self.advance();
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
