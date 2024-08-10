use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    line_number: u32,
    pub tokens: Vec<Token>,
}

impl Lexer {
    const KEYWORDS: [(&'static str, TokenType); 14] = [
        ("true", TokenType::Boolean),
        ("false", TokenType::Boolean),
        ("nil", TokenType::Nil),
        ("print", TokenType::Print),
        ("make", TokenType::Make),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("and", TokenType::And),
        ("or", TokenType::Or),
        ("while", TokenType::While),
        ("funk", TokenType::Funk),
        ("return", TokenType::Return),
        ("with", TokenType::With),
        ("as", TokenType::As),
    ];

    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            tokens: vec![],
            line_number: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                self.add_token(TokenType::EOF, "".to_string());
                break;
            }

            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line_number += 1;
                }
                '(' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::LParen, c);
                }
                ')' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::RParen, c);
                }
                '{' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::LBrace, c);
                }
                '}' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::RBrace, c);
                }
                '-' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::Minus, c);
                }
                '+' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::Plus, c);
                }
                '*' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::Star, c);
                }
                '!' => {
                    let mut c = self.advance().to_string();
                    if self.peek() == '=' {
                        c.push(self.advance());
                        self.add_token(TokenType::BangEqual, c);
                    } else {
                        self.add_token(TokenType::Bang, c);
                    }
                }
                '=' => {
                    let mut c = self.advance().to_string();
                    if self.peek() == '=' {
                        c.push(self.advance());
                        self.add_token(TokenType::EqualEqual, c);
                    } else {
                        self.add_token(TokenType::Equal, c);
                    }
                }
                '<' => {
                    let mut c = self.advance().to_string();
                    if self.peek() == '=' {
                        c.push(self.advance());
                        self.add_token(TokenType::LessEqual, c);
                    } else {
                        self.add_token(TokenType::Less, c);
                    }
                }
                '>' => {
                    let mut c = self.advance().to_string();
                    if self.peek() == '=' {
                        c.push(self.advance());
                        self.add_token(TokenType::GreaterEqual, c);
                    } else {
                        self.add_token(TokenType::Greater, c);
                    }
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        let c = self.advance().to_string();
                        self.add_token(TokenType::Slash, c);
                    }
                }
                '"' | '\'' | '`' => {
                    self.handle_string(self.peek());
                }
                ',' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::Comma, c);
                }
                ';' => {
                    let c = self.advance().to_string();
                    self.add_token(TokenType::Semicolon, c);
                }
                _ => {
                    if self.peek().is_ascii_alphabetic() {
                        self.handle_alpha();
                    } else if self.peek().is_ascii_digit() {
                        self.handle_number();
                    } else {
                        panic!("Unexpected character: {}", self.peek());
                    }
                }
            }
        }

        self.tokens.clone()
    }

    fn handle_string(&mut self, character: char) {
        self.advance();

        let mut value = String::new();

        while !self.is_at_end() && self.peek() != character && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line_number += 1;
            }
            value.push(self.advance());
        }

        if self.is_at_end() {
            panic!("Unterminated string");
        }

        self.advance();

        self.add_token(TokenType::String, value);
    }

    fn handle_number(&mut self) {
        let mut value = self.advance().to_string();

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        if self.is_at_end() {
            self.add_token(TokenType::Number, value);
            return;
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance());

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        self.add_token(TokenType::Number, value);
    }

    fn handle_alpha(&mut self) {
        let mut value = self.advance().to_string();

        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            value.push(self.advance());
        }

        let token_type = match Self::KEYWORDS.iter().find(|(k, _)| *k == value) {
            Some((_, token_type)) => token_type.clone(),
            None => TokenType::Identifier,
        };

        self.add_token(token_type, value);
    }

    fn is_at_end(&self) -> bool {
        return self.position >= self.input.len();
    }

    fn advance(&mut self) -> char {
        let current_char = self.peek();
        self.position += 1;
        return current_char;
    }

    fn peek(&self) -> char {
        return self
            .input
            .chars()
            .nth(self.position)
            .expect("Unexpected end of input");
    }

    fn peek_next(&self) -> char {
        return self
            .input
            .chars()
            .nth(self.position + 1)
            .expect("Unexpected end of input");
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line_number,
        })
    }
}
