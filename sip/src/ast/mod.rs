use crate::tokens;
use std::rc::Rc;

pub trait Node {
    fn token_literal() -> String;
}

pub enum Expression {}

pub enum Statement {
    Identifier(tokens::Token),
    VarStmt(tokens::Token, Expression), // var x = value
    Assign(tokens::Token, Expression),  // x = value

    // left, operator, right
    Logical(Expression, tokens::Token, Expression),

    // left, op, right: x + y
    Binary(Expression, tokens::Token, Expression),

    // op, value, -1, !true
    Unary(tokens::Token, Expression),

    // interger, string, true, false, etc
    Literal(tokens::Token),
}

pub struct Program {
    stmts: Vec<Statement>,
}

impl Node for Program {
    fn token_literal() -> String {
        "".to_string()
    }
}
