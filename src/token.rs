use std::fmt;
use std::string::String;

#[derive(Debug, Clone, PartialEq)]
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
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl From<&Literal> for String {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
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
