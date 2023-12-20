use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    ast::{Node, Program},
    errors::EvalError,
    object::Object,
    tokens::Token,
};

pub struct Interpreter {
    env: HashMap<String, Object>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn get_value(&self, key: String) -> Option<&Object> {
        self.env.get(&key)
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Object, EvalError> {
        let mut r = Object::Null;
        for node in program.stmts {
            r = self.eval(node)?;
        }
        Ok(r)
    }

    pub fn eval(&mut self, node: Node) -> Result<Object, EvalError> {
        match node {
            Node::Null => Ok(Object::Null),
            Node::Literal(tk) => self.eval_literal(tk),
            Node::Assign(name, node) => self.eval_assign(name, node),
            Node::Binary(left, tk, right) => self.eval_binary(Node::Binary(left, tk, right)),
            _ => Err(EvalError::UnknowNode(node)),
        }
    }

    fn eval_literal(&self, tk: Token) -> Result<Object, EvalError> {
        match tk {
            Token::Integer(v) => Ok(Object::Integer(v)),
            Token::Float(v) => Ok(Object::Float(v)),
            Token::SString(v) => Ok(Object::SString(v)),
            Token::True => Ok(Object::Bool(true)),
            Token::False => Ok(Object::Bool(false)),
            _ => Err(EvalError::NotLiteral(tk)),
        }
    }

    fn eval_assign(&mut self, name: Token, value: Rc<Node>) -> Result<Object, EvalError> {
        match name {
            Token::Ident(ident) => {
                let node = Rc::try_unwrap(value);
                let val = self.eval(node.unwrap())?;
                self.env.insert(ident, val.clone());
                Ok(val)
            }
            _ => Err(EvalError::TkIsNotIdent(name)),
        }
    }

    fn eval_binary(&mut self, bin: Node) -> Result<Object, EvalError> {
        match bin {
            Node::Binary(left, tk, right) => {
                let l_node = Rc::try_unwrap(left);
                let left_obj = self.eval(l_node.unwrap())?;

                let r_node = Rc::try_unwrap(right);
                let right_obj = self.eval(r_node.unwrap())?;

                match tk {
                    Token::Plus(_) | Token::Minus(_) | Token::Slash(_) | Token::Star(_) => {
                        self.eval_number(left_obj, tk, right_obj)
                    }
                    Token::Lt(_)
                    | Token::LtEQ(_)
                    | Token::Gt(_)
                    | Token::GtEQ(_)
                    | Token::EQ(_)
                    | Token::NotEQ(_) => self.eval_compare(left_obj, tk, right_obj),
                    _ => Err(EvalError::NotSupportedOperator(tk)),
                }
            }
            _ => Err(EvalError::UnknowNode(bin)),
        }
    }

    fn eval_compare(&self, left: Object, tk: Token, right: Object) -> Result<Object, EvalError> {
        Ok(Object::Bool(true))
    }

    fn eval_number(&self, left: Object, tk: Token, right: Object) -> Result<Object, EvalError> {
        match tk {
            Token::Plus(_) => self.eval_plus(left, right),
            _ => Err(EvalError::NotSupportedOperator(tk)),
        }
    }

    fn eval_plus(&self, left: Object, right: Object) -> Result<Object, EvalError> {
        let mut left_num: f64 = 0.0;
        let mut right_num: f64 = 0.0;
        match left {
            Object::Integer(v) => left_num = v as f64,
            Object::Float(v) => left_num = v,
            _ => return Err(EvalError::NotNumber(left)),
        }

        match right {
            Object::Integer(v) => right_num = v as f64,
            Object::Float(v) => left_num = v,
            _ => return Err(EvalError::NotNumber(right)),
        }

        Ok(Object::Number(left_num + right_num))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Node, errors::EvalError, eval::Interpreter, object::Object, tokens::Token};
    use std::rc::Rc;

    #[test]
    fn test_eval_integer() {
        let n = Node::Literal(Token::Integer(100));
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        assert_eq!(v.is_ok(), true);
        assert_eq!(Object::Integer(100), v.unwrap());
    }

    #[test]
    fn test_eval_literal1() {
        let n = Node::Literal(Token::SString("Hello, World!".to_string()));
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        assert_eq!(v.is_ok(), true);
        assert_eq!(Object::SString("Hello, World!".to_string()), v.unwrap());
    }

    #[test]
    fn test_eval_literals() {
        let nodes = vec![
            Node::Literal(Token::True),
            Node::Literal(Token::False),
            Node::Literal(Token::Integer(1024)),
            Node::Literal(Token::Float(1024.8)),
            Node::Literal(Token::SString("Hi".to_string())),
        ];
        let expect_obj = vec![
            Object::Bool(true),
            Object::Bool(false),
            Object::Integer(1024),
            Object::Float(1024.8),
            Object::SString("Hi".to_string()),
        ];
        let mut intpter = Interpreter::new();

        let mut idx = 0 as usize;
        for node in nodes {
            let v = intpter.eval(node);
            assert_eq!(v.is_ok(), true);
            assert_eq!(expect_obj[idx], v.unwrap());
            idx += 1;
        }
    }

    #[test]
    fn test_eval_assign() {
        let n = Node::Assign(
            Token::Ident("x".to_string()),
            Rc::new(Node::Literal(Token::Integer(1024))),
        );
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        assert_eq!(v.is_ok(), true);
        assert_eq!(Object::Integer(1024), v.unwrap());

        let val = intpter.get_value("x".to_string());
        println!("{:?}", val);
        assert_eq!(val.is_some(), true);
        assert_eq!(*val.unwrap(), Object::Integer(1024));
    }

    #[test]
    fn test_eval_assign1() {
        let n = Node::Assign(
            Token::SString("x".to_string()),
            Rc::new(Node::Literal(Token::Integer(1024))),
        );
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        assert_eq!(v.is_err(), true);
        assert_eq!(
            v.err(),
            Some(EvalError::TkIsNotIdent(Token::SString("x".to_string())))
        );
    }
}
