#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number,
    Identifier,
    String,
    Boolean,
    Nil,

    // Keywords
    Print,
    Make,
    If,
    Else,
    And,
    Or,
    While,

    // Single-character tokens
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Equal,
    Bang,
    Less,
    Greater,
    Comma,
    Semicolon,

    // double-character tokens
    EqualEqual,
    BangEqual,
    GreaterEqual,
    LessEqual,

    // EOF
    EOF,
}
