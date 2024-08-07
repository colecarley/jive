pub mod token {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub line: u32,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum TokenType {
        Number,
        Plus,
        Minus,
        Star,
        Slash,
        LParen,
        RParen,
        Equal,
        EqualEqual,
        Bang,
        BangEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,
        AndAnd,
        OrOr,
        Identifier,
        String,
        Boolean,
        Nil,
        EOF,
    }
}
