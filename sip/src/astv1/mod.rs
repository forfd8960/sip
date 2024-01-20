use std::rc::Rc;

use crate::tokens::Token;

pub trait Node {
    fn typ(&self) -> NodeType {
        NodeType::Null
    }
    fn as_ident(&self) -> Option<Ident> {
        None
    }
    fn as_var(&self) -> Option<VarStmt> {
        None
    }
    fn as_assign(&self) -> Option<Assign> {
        None
    }
    fn as_if(&self) -> Option<IfStmt> {
        None
    }
    fn as_logical(&self) -> Option<Logical> {
        None
    }
    fn as_binary(&self) -> Option<Binary> {
        None
    }
    fn as_unary(&self) -> Option<Unary> {
        None
    }
    fn as_literal(&self) -> Option<Literal> {
        None
    }
    fn as_group(&self) -> Option<Group> {
        None
    }
    fn as_block(&self) -> Option<Block> {
        None
    }
    fn as_null(&self) -> Option<Null> {
        Some(Null {})
    }
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Identifier,
    VarStmt, // var x = value
    Assign,  // x = value
    IfStmt,  // cond, then, elseThen
    Logical, // left, operator, right
    Binary,  // left, op, right: x + y
    Unary,   // // op, value, -1, !true
    Literal, // interger, string, true, false, etc
    Group,
    Block,
    Null,
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub name: Token,
}

impl Ident {
    pub fn new(name: Token) -> Self {
        Self { name: name }
    }
}

impl Node for Ident {
    fn as_ident(&self) -> Option<Ident> {
        Some(Ident {
            name: self.name.clone(),
        })
    }
}

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub value: Rc<Box<dyn Node>>,
}

impl VarStmt {
    pub fn new(name: Token, value: Box<dyn Node>) -> Self {
        Self {
            name: name,
            value: Rc::new(value),
        }
    }
}

impl Node for VarStmt {
    fn as_var(&self) -> Option<VarStmt> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Rc<Box<dyn Node>>,
}

impl Assign {
    pub fn new(name: Token, value: Box<dyn Node>) -> Self {
        Self {
            name: name,
            value: Rc::new(value),
        }
    }
}

impl Node for Assign {
    fn as_assign(&self) -> Option<Assign> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct IfStmt {
    pub cond: Rc<Box<dyn Node>>,
    pub else_stmt: Rc<Box<dyn Node>>,
    pub then_stmt: Rc<Box<dyn Node>>,
}

impl IfStmt {
    pub fn new(cond: Box<dyn Node>, else_stmt: Box<dyn Node>, then_stmt: Box<dyn Node>) -> Self {
        Self {
            cond: Rc::new(cond),
            else_stmt: Rc::new(else_stmt),
            then_stmt: Rc::new(then_stmt),
        }
    }
}

impl Node for IfStmt {
    fn as_if(&self) -> Option<IfStmt> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Logical {
    pub left: Rc<Box<dyn Node>>,
    pub op: Token,
    pub right: Rc<Box<dyn Node>>,
}

impl Logical {
    pub fn new(left: Box<dyn Node>, op: Token, right: Box<dyn Node>) -> Self {
        Self {
            left: Rc::new(left),
            op,
            right: Rc::new(right),
        }
    }
}

impl Node for Logical {
    fn as_logical(&self) -> Option<Logical> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Binary {
    pub op: Token,
    pub left: Rc<Box<dyn Node>>,
    pub right: Rc<Box<dyn Node>>,
}

impl Binary {
    pub fn new(left: Box<dyn Node>, op: Token, right: Box<dyn Node>) -> Self {
        Self {
            op,
            left: Rc::new(left),
            right: Rc::new(right),
        }
    }
}

impl Node for Binary {
    fn as_binary(&self) -> Option<Binary> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Unary {
    pub op: Token,
    pub right: Rc<Box<dyn Node>>,
}

impl Unary {
    pub fn new(op: Token, right: Box<dyn Node>) -> Self {
        Self {
            op: op,
            right: Rc::new(right),
        }
    }
}

impl Node for Unary {
    fn as_unary(&self) -> Option<Unary> {
        Some(self.clone())
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

impl Node for Literal {
    fn as_literal(&self) -> Option<Literal> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Group {
    pub stmt: Rc<Box<dyn Node>>,
}

impl Group {
    pub fn new(stmt: Box<dyn Node>) -> Self {
        Self {
            stmt: Rc::new(stmt),
        }
    }
}

impl Node for Group {
    fn as_group(&self) -> Option<Group> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Block {
    pub stmts: Vec<Rc<Box<dyn Node>>>,
}

impl Block {
    pub fn new(stmts: Vec<Box<dyn Node>>) -> Self {
        let mut bs = vec![];
        for stmt in stmts {
            bs.push(Rc::new(stmt))
        }
        Self { stmts: bs }
    }
}

impl Node for Block {
    fn as_block(&self) -> Option<Block> {
        Some(self.clone())
    }
}

#[derive(Clone)]
pub struct Null {}

impl Null {
    pub fn new() -> Self {
        Null {}
    }
}

impl Node for Null {
    fn as_null(&self) -> Option<Null> {
        Some(self.clone())
    }
}

pub struct Program {
    pub stmts: Vec<Box<dyn Node>>,
}

impl Program {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { stmts: nodes }
    }
}
