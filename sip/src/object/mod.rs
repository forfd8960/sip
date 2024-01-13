use core::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Class(String),
    ClassInstance(String),
    Function(String),
    Integer(i64),
    Float(f64),
    Number(f64),
    Bool(bool),
    SString(String),
    Return(Rc<Object>),
    Print(Vec<Object>),
    Error(String),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(v) => write!(f, "{}", v),
            Object::Float(v) => write!(f, "{}", v),
            Object::Number(v) => write!(f, "{}", v),
            Object::SString(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            _ => write!(f, "{:?}", self),
        }
    }
}
