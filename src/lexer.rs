use crate::token::{Literal, Token, TokenType};
use phf::{phf_map, Map};

static KEYWORDS: Map<&'static str, TokenType> = phf_map! {
    "and"  => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "fun"  => TokenType::Fun,
    "for"  => TokenType::For,
    "if"  => TokenType::If,
    "nil"  => TokenType::Nil,
    "or"   => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var"  => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Lexer {
    // TODO: probably want to make source a buffered reader
    source: String,
    tokens: Vec<Token>,
    errored: bool,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let tokens = Vec::new();
        Lexer {
            source,
            tokens,
            errored: false,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn lex_tokens(mut self) -> Vec<Token> {
        while !self.at_end() {
            // Once we have finished lexing a token, we set our start position to be aligned
            // with the current position, this is similar to a two cursor solution to
            // linked list problems
            self.start = self.current;
            self.lex_token()
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            None,
            self.line,
        ));

        self.tokens
    }

    fn lex_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => {
                    let t = if self.match_next('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(t, None);
                }
                '=' => {
                    let t = if self.match_next('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(t, None);
                }
                '>' => {
                    let t = if self.match_next('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(t, None);
                }
                '<' => {
                    let t = if self.match_next('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(t, None);
                }
                '/' => {
                    if self.match_next('/') {
                        while self.peek() != '\n' && !self.at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None)
                    }
                }

                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                '"' => self.string(),

                _ => {
                    if c.is_digit(10) {
                        self.number()
                    } else if self.is_alpha(c) {
                        self.identifier()
                    } else {
                        eprintln!("Unexpected character: {} at line {}", c, self.line);
                        self.errored = true;
                    }
                }
            }
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);

        self.current += 1;

        c
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.at_end() {
            eprintln!("String was not terminated at line: {}", &self.line)
        }

        self.advance();

        // Trim surrounding quotes
        let s = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::String(String::from(s))));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Some(Literal::Number(
                self.source[self.start..self.current].parse().unwrap(),
            )),
        )
    }

    fn identifier(&mut self) {
        while self.is_alphanum(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);

        self.add_token(token_type, None)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn is_alpha(&self, c: char) -> bool {
        return c.is_alphabetic() || c == '_';
    }

    fn is_alphanum(&self, c: char) -> bool {
        c.is_digit(10) || self.is_alpha(c)
    }

    fn add_token(&mut self, t: TokenType, literal: Option<Literal>) {
        let s = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(t, String::from(s), literal, self.line));
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
