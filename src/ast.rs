use crate::token::{Token, Literal};

enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expr: Box<Expr>
    },
    Literal(Literal),
}

pub trait Visitor<T> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &Visitor<T>) -> T {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Grouping { expr } => visitor.visit_grouping(expr),
            Expr::Literal { literal } => visitor.visit_grouping(literal),
        }
    }
}
