use crate::ast::Expr;
use crate::token::{Token, Literal};
use crate::token::TokenType as Tk;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        // TODO: This feels jank
        while self.match_next(vec![Tk::BangEqual, Tk::EqualEqual]) {
            let operator = self.previous();
            let right = Box::new(self.comparison());
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_next(vec![Tk::Greater, Tk::GreaterEqual, Tk::Less, Tk::LessEqual]) {
            let operator = self.previous();
            let right = Box::new(self.term());
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_next(vec![Tk::Minus, Tk::Plus]) {
            let operator = self.previous();
            let right = Box::new(self.factor());
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_next(vec![Tk::Slash, Tk::Star]) {
            let operator = self.previous();
            let right = Box::new(self.unary());
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_next(vec![Tk::Bang, Tk::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_next(vec![Tk::False]) {
            return Expr::Literal(Literal::String(String::from("false")));
        }
        if self.match_next(vec![Tk::True]) {
            return Expr::Literal(Literal::String(String::from("true")));
        }
        if self.match_next(vec![Tk::Nil]) {
            return Expr::Literal(Literal::String(String::from("null")));
        }

        if self.match_next(vec![Tk::Number, Tk::String]) {
            return Expr::Literal(self.previous().literal.unwrap());
        }

        if self.match_next(vec![Tk::LeftParen]) {
            let expr = self.expression();
            self.consume(Tk::RightParen, "Expect ')', after expression.");
            Expr::Grouping(expr)
        }
    }

    fn match_next(&mut self, types: Vec<Tk>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    // TODO: May want to refactor all of these methods to use references
    // probably want to do so with Rc<>
    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn at_end(&mut self) -> bool {
        self.peek().r#type == Tk::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1
        };
        self.previous()
    }

    fn check(&mut self, t: Tk) -> bool {
        if self.at_end() {
            return false;
        };
        self.tokens[self.current].r#type == t
    }

    fn consume(&mut self, t: Tk, s: String) {
        todo!()
    }
}
