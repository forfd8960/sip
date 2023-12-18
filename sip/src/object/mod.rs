#[derive(Debug, PartialEq)]
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
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Debug)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(v: i64) -> Self {
        Self { value: v }
    }
}

impl Object for Integer {
    fn obj_type(&self) -> ObjectType {
        ObjectType::Integer(self.value)
    }

    fn inspect(&self) -> String {
        format!("integer: {}", self.value)
    }
}

#[derive(Debug)]
pub struct Float {
    value: f64,
}

impl Object for Float {
    fn obj_type(&self) -> ObjectType {
        ObjectType::Float(self.value)
    }

    fn inspect(&self) -> String {
        format!("float: {}", self.value)
    }
}

#[derive(Debug)]
pub struct SString {
    value: String,
}

impl Object for SString {
    fn obj_type(&self) -> ObjectType {
        ObjectType::SString(self.value.clone())
    }

    fn inspect(&self) -> String {
        format!("string: {}", self.value)
    }
}

#[derive(Debug)]
pub struct Bool {
    value: bool,
}

impl Object for Bool {
    fn obj_type(&self) -> ObjectType {
        ObjectType::Bool(self.value)
    }

    fn inspect(&self) -> String {
        format!("bool: {}", self.value)
    }
}

pub struct Return<'a> {
    value: Box<&'a dyn Object>,
}

impl<'a> Object for Return<'a> {
    fn obj_type(&self) -> ObjectType {
        ObjectType::Return
    }

    fn inspect(&self) -> String {
        format!("return: {}", self.value.inspect())
    }
}
