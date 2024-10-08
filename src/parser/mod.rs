/*
program → declaration* EOF ;
statement → printStatement | expressionStatement | ifStatement | block | whileStatement | returnStatement | wihtStatement | forStatement ;
forStatement → "for" expression "in" expression statement;
withStatement → "with" expression "as" expression statement;
declaration → functionDeclaration | variableDeclaration |  statement ;
variableDeclaration → "make" IDENTIFIER ( "=" expression )? ";" ;
functionDeclaration → "funk" function ;
function → IDENTIFIER "(" parameters? ")" block ;
whileStatement → "while" expression statement ;
ifStatement → "if"  expression  statement ( "else"  statement )? ;
block → "{" declaration* "}" ;
expression → assignment ;
assignment → (call ("." | "[" NUMBER "]"))? identifier "=" assignment | ifExpression ;
ifExpression → expression "if" expression "else" expression | equality ;
or → and ( "||" and )* | and;
and → equality ( "&&" equality )* | equality;
equality → comparison ( ( "!=" | "==" ) comparison )* ;
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term → factor ( ( "-" | "+" ) factor )* ;
factor → unary ( ( "/" | "*" ) unary )* ;
unary → ("!"|"-") unary
        | call ;
call → primary ( "(" arguments? ")" )* ;
primary → NUMBER | STRING | "true" | "false" | "nil"
        | "(" expression ")" ;
 */

use crate::token::{Token, TokenType};

pub mod accept;
pub mod expression;
pub mod statement;

use expression::{
    And, Assignment, Call, Comparison, Equality, Expression, Factor, IfExpression, Index,
    IndexAssignment, List, MapIndex, MapIndexAssignment, Or, Primary, Record, Term, Unary,
};
use statement::{
    Block, ExpressionStatement, For, FunctionDeclaration, IfStatement, PrintStatement, Return,
    Statement, VariableDeclaration, WhileStatement, With,
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
            statements.push(self.declaration());
        }

        return statements;
    }

    fn declaration(&mut self) -> Statement {
        match self.peek().token_type {
            TokenType::Make => return self.declaration_statement(),
            TokenType::Funk => return self.function_declaration(),
            _ => return self.statement(),
        }
    }

    fn statement(&mut self) -> Statement {
        match self.peek().token_type {
            TokenType::Print => return self.print_statement(),
            TokenType::LBrace => return self.block(),
            TokenType::If => return self.if_statement(),
            TokenType::While => return self.while_statement(),
            TokenType::Return => return self.return_statement(),
            TokenType::With => return self.with_statement(),
            TokenType::For => return self.for_statement(),
            _ => return self.expression_statement(),
        }
    }

    fn for_statement(&mut self) -> Statement {
        self.advance();

        let identifier = self.peek();
        self.advance();

        if self.peek().token_type != TokenType::In {
            panic!("Expected 'as' keyword after 'with' keyword");
        }

        self.advance();

        return Statement::For(Box::new(For {
            identifier,
            iter: self.expression(),
            body: self.statement(),
        }));
    }

    fn with_statement(&mut self) -> Statement {
        self.advance();
        let value = self.expression();

        if self.peek().token_type != TokenType::As {
            panic!("Expected 'as' keyword after 'with' keyword");
        }
        self.advance();

        return Statement::With(Box::new(With {
            value,
            identifier: self.advance(),
            body: self.statement(),
        }));
    }

    fn return_statement(&mut self) -> Statement {
        self.advance();

        if self.peek().token_type == TokenType::Semicolon {
            self.advance();
            return Statement::Return(Box::new(Return { value: None }));
        }

        let value = self.expression();

        if self.peek().token_type != TokenType::Semicolon {
            panic!("Expected ';' after return statement");
        }

        self.advance();

        return Statement::Return(Box::new(Return { value: Some(value) }));
    }

    fn while_statement(&mut self) -> Statement {
        self.advance();
        return Statement::WhileStatement(Box::new(WhileStatement {
            condition: self.expression(),
            body: self.statement(),
        }));
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
            statements.push(self.declaration());
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

    fn function_declaration(&mut self) -> Statement {
        self.advance();

        let identifier = self.advance();

        if self.peek().token_type != TokenType::LParen {
            panic!("Expected '(' after function identifier");
        }

        self.advance();

        let mut parameters = Vec::<Token>::new();

        if self.peek().token_type != TokenType::RParen {
            parameters.push(self.advance());

            while self.peek().token_type == TokenType::Comma {
                self.advance();
                parameters.push(self.advance());
            }
        }

        if self.peek().token_type != TokenType::RParen {
            panic!("Expected ')' after parameters");
        }

        self.advance();

        let body = self.block();

        return Statement::FunctionDeclaration(Box::new(FunctionDeclaration {
            identifier,
            parameters,
            body,
        }));
    }

    fn declaration_statement(&mut self) -> Statement {
        self.advance();

        let identifier = self.advance();

        if self.peek().token_type == TokenType::Semicolon {
            self.advance();
            return Statement::VariableDeclaration(Box::new(VariableDeclaration {
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

        return Statement::VariableDeclaration(Box::new(VariableDeclaration {
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
                Expression::MapIndex(map_index) => {
                    return Expression::MapIndexAssignment(Box::new(MapIndexAssignment {
                        map: map_index.map,
                        key: map_index.key,
                        value: second,
                    }))
                }
                Expression::Index(index) => {
                    return Expression::IndexAssignment(Box::new(IndexAssignment {
                        list: index.list,
                        expression: index.expression,
                        value: second,
                    }))
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
            return self.call();
        }
    }

    fn call(&mut self) -> Expression {
        let mut identifier = self.primary();

        while self.peek().token_type == TokenType::LParen
            || self.peek().token_type == TokenType::LBracket
            || self.peek().token_type == TokenType::Dot
        {
            if self.peek().token_type == TokenType::LParen {
                self.advance();
                let arguments = self.arguments();
                identifier = Expression::Call(Box::new(Call {
                    identifier,
                    arguments,
                }));
            }

            if self.peek().token_type == TokenType::LBracket {
                self.advance();
                let expression = self.expression();
                if self.peek().token_type != TokenType::RBracket {
                    panic!("Expected '] after list index")
                }
                self.advance();

                identifier = Expression::Index(Box::new(Index {
                    list: identifier,
                    expression,
                }))
            }

            if self.peek().token_type == TokenType::Dot {
                self.advance();
                let key = self.peek();
                self.advance();

                identifier = Expression::MapIndex(Box::new(MapIndex {
                    map: identifier,
                    key,
                }))
            }
        }

        identifier
    }

    fn arguments(&mut self) -> Vec<Expression> {
        let mut arguments = vec![];

        if self.peek().token_type != TokenType::RParen {
            arguments.push(self.expression());

            while self.peek().token_type == TokenType::Comma {
                self.advance();
                arguments.push(self.expression());
            }

            if self.peek().token_type != TokenType::RParen {
                panic!("Expected ')' after arguments");
            }
        }

        self.advance();
        return arguments;
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
            TokenType::LBracket => {
                return self.list();
            }
            TokenType::LBrace => {
                return self.record();
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
            _ => panic!("Unexpected token, {:?}", self.advance().token_type),
        }
    }

    fn record(&mut self) -> Expression {
        self.advance();

        let mut key_values = Vec::<(Token, Expression)>::new();
        while self.peek().token_type != TokenType::RBrace {
            let key = self.peek();
            self.advance();

            if self.peek().token_type != TokenType::Colon {
                panic!(
                    "expected semicolon after key in record, found {:?}",
                    self.peek().token_type
                );
            }

            self.advance();

            let value = self.expression();

            if self.peek().token_type != TokenType::Comma {
                panic!("expected ',' after key value pair in record");
            }

            self.advance();

            key_values.push((key, value));
        }

        self.advance();

        return Expression::Record(Box::new(Record { key_values }));
    }

    fn list(&mut self) -> Expression {
        self.advance();

        let mut values = Vec::new();
        if self.peek().token_type != TokenType::RBracket {
            let expression = self.expression();
            values.push(expression);

            while self.peek().token_type == TokenType::Comma {
                self.advance();
                let expression = self.expression();
                values.push(expression);
            }

            if self.peek().token_type != TokenType::RBracket {
                panic!("Unterminated list");
            }
        }

        self.advance();

        return Expression::List(Box::new(List { values }));
    }

    fn advance(&mut self) -> Token {
        self.position += 1;
        self.tokens[self.position - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
}
