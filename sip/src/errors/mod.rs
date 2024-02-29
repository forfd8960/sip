use crate::{ast::Node, object::Object, tokens::Token};

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(char),
    InvalidString(String),
    InvalidNum(String),
}

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidToken(msg) => write!(f, "{}", msg),
            LexerError::InvalidString(msg) => write!(f, "{}", msg),
            LexerError::InvalidNum(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    NotSupportedToken(Token),
    ExpectedTokenNotFound(String),
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NotSupportedToken(tk) => write!(f, "{:?}", tk),
            ParserError::ExpectedTokenNotFound(s) => write!(f, "{}", s),
        }
    }
}

/*
    ErrDivideByZero                  = "integer divide by zero"
    ErrNotSupportedOperator          = "operator is not supported: %v"
    ErrIdentifierNotFound            = "identifier: %s is not found"
    ErrIdentifierIsNotCallable       = "%s is not callable(it shoud be function or xxx)"
    ErrOnlyClassInstanceHaveProperty = "expr: %s can not get property, only class instance have property"
*/
#[derive(Debug, PartialEq)]
pub enum EvalError {
    NotLiteral(Token),
    NotNumber(Object),
    NotIdent(Token),
    NotNumberOrStr(Object),
    DifferObjectToCompare(Object, Object),
    DivideByZero(String),
    NotSupportedOperator(Token),
    TkIsNotIdent(Token),
    IdentNotFound(String),
    IdentifierIsNotCallable(String),
    OnlyClassInstanceHaveProperty(String),
    UnknowNode(Node),
    EmptyNode,
}

impl std::error::Error for EvalError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::NotLiteral(tk) => write!(f, "{:?}", tk),
            EvalError::NotNumber(obj) => write!(f, "{:?} is not number", obj),
            EvalError::NotIdent(tk) => write!(f, "{:?} is not identifier", tk),
            EvalError::NotNumberOrStr(obj) => write!(f, "{:?} is not number or string", obj),
            EvalError::DifferObjectToCompare(obj1, obj2) => {
                write!(f, "{:?}, {:?} are different", obj1, obj2)
            }
            EvalError::DivideByZero(s) => write!(f, "{}", s),
            EvalError::NotSupportedOperator(c) => write!(f, "operator: {:?} is not supported", c),
            EvalError::IdentNotFound(ident) => write!(f, "identifier: {} is not found", ident),
            EvalError::TkIsNotIdent(tk) => write!(f, "token: {:?} is not identifier", tk),
            EvalError::IdentifierIsNotCallable(ident) => {
                write!(f, "{} is not callable(it shoud be function or xxx)", ident)
            }
            EvalError::OnlyClassInstanceHaveProperty(s) => {
                write!(
                    f,
                    "callee: {} can not get property, only class instance have property",
                    s
                )
            }
            EvalError::UnknowNode(node) => {
                write!(f, "unknown node: {:?}", node.clone())
            }
            EvalError::EmptyNode => {
                write!(f, "empty node")
            }
        }
    }
}
