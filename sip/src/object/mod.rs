pub enum ObjectType {
    Class(String),
    ClassInstance(String),
    Function(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    SString(String),
    Return,
    Print,
    Error,
    Null,
}

pub trait Object {
    fn obj_type() -> ObjectType;
    fn inspect() -> String;
}
