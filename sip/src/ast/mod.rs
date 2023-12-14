use crate::tokens;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Identifier(tokens::Token),
    VarStmt(tokens::Token, Rc<Node>), // var x = value
    Assign(tokens::Token, Rc<Node>),  // x = value

    // left, operator, right
    Logical(Rc<Node>, tokens::Token, Rc<Node>),

    // left, op, right: x + y
    Binary(Rc<Node>, tokens::Token, Rc<Node>),

    // op, value, -1, !true
    Unary(tokens::Token, Rc<Node>),
    ExpressionStmt(Rc<Node>),
    // interger, string, true, false, etc
    Literal(tokens::Token),
    Group(Rc<Node>),
}

#[derive(Debug, Clone)]
pub struct Program {
    stmts: Vec<Node>,
}

impl Program {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { stmts: nodes }
    }
}
