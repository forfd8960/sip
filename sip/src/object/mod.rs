use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Class(String),
    ClassInstance(String),
    Function(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    SString(String),
    Return(Rc<Object>),
    Print(Vec<Object>),
    Error(String),
    Null,
}
