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


