use crate::tokens::{self, Token};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Identifier(tokens::Token),
    VarStmt(VarStmt),                     // var x = value
    Assign(Assign),                       // x = value
    IfStmt(Rc<Node>, Rc<Node>, Rc<Node>), // cond, then, elseThen

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
    Return(Return),
    Block(Vec<Node>),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarStmt {
    pub name: Token,
    pub value: Rc<Node>,
}

impl VarStmt {
    pub fn new(name: Token, value: Node) -> Self {
        Self {
            name: name,
            value: Rc::new(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub name: Token,
    pub value: Rc<Node>,
}

impl Assign {
    pub fn new(name: Token, value: Node) -> Self {
        Self {
            name: name,
            value: Rc::new(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStmt {
    pub cond: Rc<Node>,
    pub else_stmt: Rc<Node>,
    pub then_stmt: Rc<Node>,
}

impl IfStmt {
    pub fn new(cond: Node, else_stmt: Node, then_stmt: Node) -> Self {
        Self {
            cond: Rc::new(cond),
            else_stmt: Rc::new(else_stmt),
            then_stmt: Rc::new(then_stmt),
        }
    }
}

#[derive(Clone)]
pub struct Logical {
    pub left: Rc<Node>,
    pub op: Token,
    pub right: Rc<Node>,
}

impl Logical {
    pub fn new(left: Node, op: Token, right: Node) -> Self {
        Self {
            left: Rc::new(left),
            op,
            right: Rc::new(right),
        }
    }
}

#[derive(Clone)]
pub struct Binary {
    pub op: Token,
    pub left: Rc<Node>,
    pub right: Rc<Node>,
}

impl Binary {
    pub fn new(left: Node, op: Token, right: Node) -> Self {
        Self {
            op,
            left: Rc::new(left),
            right: Rc::new(right),
        }
    }
}

#[derive(Clone)]
pub struct Unary {
    pub op: Token,
    pub right: Rc<Node>,
}

impl Unary {
    pub fn new(op: Token, right: Node) -> Self {
        Self {
            op: op,
            right: Rc::new(right),
        }
    }
}

#[derive(Clone)]
pub struct Literal {
    pub literal: Token,
}

impl Literal {
    pub fn new(l: Token) -> Self {
        Self { literal: l }
    }
}

#[derive(Clone)]
pub struct Group {
    pub stmt: Rc<Node>,
}

impl Group {
    pub fn new(stmt: Node) -> Self {
        Self {
            stmt: Rc::new(stmt),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Rc<Node>,
}

impl Return {
    pub fn new(value: Node) -> Self {
        Self {
            value: Rc::new(value),
        }
    }
}

#[derive(Clone)]
pub struct Block {
    pub stmts: Vec<Node>,
}

impl Block {
    pub fn new(stmts: Vec<Node>) -> Self {
        let mut bs = vec![];
        for stmt in stmts {
            bs.push(stmt)
        }
        Self { stmts: bs }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Node>,
}

impl Program {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { stmts: nodes }
    }
}
