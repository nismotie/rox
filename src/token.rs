use std::string::String;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Char Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // Comparison tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literal tokens
    Identifier,
    String,
    Number,

    // Keyword Tokens
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Escape tokens
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Token {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl From<Token> for String {
    fn from(t: Token) -> String {
        format!("{:?} {} {:?}", t.r#type, t.lexeme, t.literal)
    }
}
