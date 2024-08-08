pub mod token {
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

        // Single-character tokens
        Plus,
        Minus,
        Star,
        Slash,
        LParen,
        RParen,
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
        AndAnd,
        OrOr,

        // EOF
        EOF,
    }
}
