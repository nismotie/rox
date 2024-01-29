use crate::token::{Literal, Token, TokenType};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal(Literal),
}

pub trait Visitor<T> {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&self, expr: &Expr) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Grouping { expr } => visitor.visit_grouping(expr),
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}

struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping(&self, expr: &Expr) -> String {
        self.parenthesize("group".to_string(), vec![expr])
    }

    fn visit_literal(&self, literal: &Literal) -> String {
        literal.to_string()
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

impl AstPrinter {
    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let mut s = String::new();

        s.push('(');
        s.push_str(&name);
        for e in exprs {
            s.push(' ');
            s.push_str(&e.accept(self));
        }
        s.push(')');
        s
    }

    fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }
}

#[test]
fn test_print() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, '-'.to_string(), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(123.))),
        }),
        operator: Token::new(TokenType::Star, '*'.to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expr: Box::new(Expr::Literal(Literal::Number(45.67))),
        }),
    };

    let printer = AstPrinter;

    assert_eq!(printer.print(expr), "(* (- 123) (group 45.67))");
}
