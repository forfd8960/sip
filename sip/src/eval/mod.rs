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

    fn set_value(&mut self, key: String, val: Object) {
        self.env.insert(key, val);
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
            Node::Unary(unary) => {
                let node = (*unary.right).clone();
                self.eval_unary(unary.op, node)
            }
            Node::VarStmt(var_stmt) => {
                let node = (*var_stmt.value).clone();
                self.eval_var_stmt(var_stmt.name, node)
            }
            Node::IfStmt(cond, then_stmt, else_stmt) => {
                let cond_node = (*cond).clone();
                let then_node = (*then_stmt).clone();
                let else_node = (*else_stmt).clone();
                self.eval_if(cond_node, then_node, else_node)
            }
            Node::Block(stmts) => self.eval_block(stmts),
            Node::Identifier(x) => self.eval_identifier(x),
            Node::Assign(assign) => {
                let node: Node = (*assign.value).clone();
                self.eval_assign(assign.name, node)
            }
            Node::Binary(left, tk, right) => {
                let ln = (*left).clone();
                let rn = (*right).clone();
                self.eval_binary(ln, tk, rn)
            }
            Node::Return(ret) => {
                let node = (*ret.value).clone();
                self.eval_return(node)
            }
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

    fn eval_unary(&mut self, tk: Token, node: Node) -> Result<Object, EvalError> {
        match tk {
            Token::Bang => {
                let value = self.eval(node)?;
                match value {
                    Object::Bool(v) => Ok(Object::Bool(!v)),
                    _ => Err(EvalError::NotLiteral(tk)),
                }
            }
            Token::Minus(_) => {
                let value = self.eval(node)?;
                match value {
                    Object::Integer(v) => Ok(Object::Integer(-v)),
                    Object::Float(v) => Ok(Object::Float(-v)),
                    _ => Err(EvalError::NotLiteral(tk)),
                }
            }
            _ => Err(EvalError::NotLiteral(tk)),
        }
    }

    fn eval_var_stmt(&mut self, tk: Token, value: Node) -> Result<Object, EvalError> {
        match tk {
            Token::Ident(v) => {
                let final_val = self.eval(value)?;
                self.set_value(v, final_val.clone());
                Ok(final_val)
            }
            _ => Err(EvalError::NotIdent(tk)),
        }
    }

    fn eval_if(&mut self, cond: Node, then: Node, else_node: Node) -> Result<Object, EvalError> {
        let truth = self.eval(cond)?;
        match truth {
            Object::Bool(v) => {
                if v {
                    self.eval(then)
                } else {
                    self.eval(else_node)
                }
            }
            _ => Err(EvalError::NotTruthCond(truth)),
        }
    }

    fn eval_block(&mut self, stms: Vec<Node>) -> Result<Object, EvalError> {
        let mut obj = Object::Null;
        for stmt in stms {
            obj = self.eval(stmt)?;
        }

        Ok(obj)
    }

    fn eval_identifier(&mut self, tk: Token) -> Result<Object, EvalError> {
        match tk {
            Token::Ident(ident) => {
                if let Some(v) = self.get_value(ident.clone()) {
                    return Ok(v.clone());
                } else {
                    return Err(EvalError::IdentNotFound(ident));
                }
            }
            _ => Err(EvalError::NotIdent(tk)),
        }
    }

    fn eval_assign(&mut self, name: Token, value: Node) -> Result<Object, EvalError> {
        match name {
            Token::Ident(ident) => {
                let val = self.eval(value)?;
                self.set_value(ident, val.clone());
                Ok(val)
            }
            _ => Err(EvalError::TkIsNotIdent(name)),
        }
    }

    fn eval_binary(&mut self, left: Node, tk: Token, right: Node) -> Result<Object, EvalError> {
        let left_obj = self.eval(left)?;
        let right_obj = self.eval(right)?;

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

    fn eval_compare(&self, left: Object, tk: Token, right: Object) -> Result<Object, EvalError> {
        let mut left_num: f64 = 0.0;
        let mut is_num: bool = false;
        let mut is_str: bool = false;
        let mut left_str = "".to_string();
        match left {
            Object::Integer(v) => {
                left_num = v as f64;
                is_num = true;
            }
            Object::Float(v) => {
                left_num = v;
                is_num = true;
            }
            Object::SString(ref v) => {
                left_str = v.clone();
                is_str = true;
            }
            _ => return Err(EvalError::NotNumberOrStr(left)),
        };

        let mut right_num: f64 = 0.0;
        let mut right_str = "".to_string();
        match right {
            Object::Integer(v) => {
                right_num = v as f64;
                if !is_num {
                    return Err(EvalError::DifferObjectToCompare(left, right));
                }

                self.eval_compare_num(tk, left_num, right_num)
            }
            Object::Float(v) => {
                right_num = v;
                if !is_num {
                    return Err(EvalError::DifferObjectToCompare(left, right));
                }

                self.eval_compare_num(tk, left_num, right_num)
            }
            Object::SString(ref v) => {
                right_str = v.clone();
                if !is_str {
                    return Err(EvalError::DifferObjectToCompare(left, right));
                }

                self.eval_compare_str(tk, left_str, right_str)
            }
            _ => return Err(EvalError::NotNumberOrStr(right)),
        }
    }

    fn eval_compare_num(&self, tk: Token, left: f64, right: f64) -> Result<Object, EvalError> {
        match tk {
            Token::Lt(_) => Ok(Object::Bool(left < right)),
            Token::LtEQ(_) => Ok(Object::Bool(left <= right)),
            Token::Gt(_) => Ok(Object::Bool(left > right)),
            Token::GtEQ(_) => Ok(Object::Bool(left >= right)),
            Token::EQ(_) => Ok(Object::Bool(left == right)),
            Token::NotEQ(_) => Ok(Object::Bool(left != right)),
            _ => Err(EvalError::NotSupportedOperator(tk)),
        }
    }

    fn eval_compare_str(
        &self,
        tk: Token,
        left: String,
        right: String,
    ) -> Result<Object, EvalError> {
        match tk {
            Token::Lt(_) => Ok(Object::Bool(left < right)),
            Token::LtEQ(_) => Ok(Object::Bool(left <= right)),
            Token::Gt(_) => Ok(Object::Bool(left > right)),
            Token::GtEQ(_) => Ok(Object::Bool(left >= right)),
            Token::EQ(_) => Ok(Object::Bool(left == right)),
            Token::NotEQ(_) => Ok(Object::Bool(left != right)),
            _ => Err(EvalError::NotSupportedOperator(tk)),
        }
    }

    fn eval_number(&self, left: Object, tk: Token, right: Object) -> Result<Object, EvalError> {
        match tk {
            Token::Plus(_) | Token::Minus(_) | Token::Slash(_) | Token::Star(_) => {
                self.eval_num_binary(tk, left, right)
            }
            _ => Err(EvalError::NotSupportedOperator(tk)),
        }
    }

    fn eval_num_binary(&self, tk: Token, left: Object, right: Object) -> Result<Object, EvalError> {
        let mut left_num: f64 = 0.0;
        let mut right_num: f64 = 0.0;
        match left {
            Object::Integer(v) => left_num = v as f64,
            Object::Number(v) => left_num = v,
            Object::Float(v) => left_num = v,
            _ => return Err(EvalError::NotNumber(left)),
        }

        match right {
            Object::Integer(v) => right_num = v as f64,
            Object::Number(v) => right_num = v,
            Object::Float(v) => right_num = v,
            _ => return Err(EvalError::NotNumber(right)),
        }

        match tk {
            Token::Plus(_) => Ok(Object::Number(left_num + right_num)),
            Token::Minus(_) => Ok(Object::Number(left_num - right_num)),
            Token::Star(_) => Ok(Object::Number(left_num * right_num)),
            Token::Slash(_) => {
                if right_num == 0.0 {
                    return Err(EvalError::DivideByZero(format!(
                        "right: {:?} num is zero",
                        right
                    )));
                }
                Ok(Object::Number(left_num / right_num))
            }
            _ => Err(EvalError::NotSupportedOperator(tk)),
        }
    }

    fn eval_return(&mut self, ret_value: Node) -> Result<Object, EvalError> {
        let obj = self.eval(ret_value)?;
        Ok(Object::Return(Rc::new(obj)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{self, Node},
        errors::EvalError,
        eval::Interpreter,
        object::Object,
        tokens::Token,
    };
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
        let n = Node::Assign(ast::Assign::new(
            Token::Ident("x".to_string()),
            Node::Literal(Token::Integer(1024)),
        ));
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
        let n = Node::Assign(ast::Assign::new(
            Token::Integer(1),
            Node::Literal(Token::Integer(1024)),
        ));
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        assert_eq!(v.is_err(), true);
        assert_eq!(
            v.err(),
            Some(EvalError::TkIsNotIdent(Token::SString("x".to_string())))
        );
    }

    #[test]
    fn test_eval_binary() {
        let n = Node::Binary(
            Rc::new(Node::Literal(Token::Integer(1024))),
            Token::Plus('+'),
            Rc::new(Node::Literal(Token::Integer(1024))),
        );
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        println!("obj: {:?}", v);
        assert_eq!(v.is_ok(), true);
        assert_eq!(Object::Number(2048 as f64), v.unwrap());

        let v1 = intpter.eval(Node::Binary(
            Rc::new(Node::Literal(Token::Integer(1))),
            Token::Minus('-'),
            Rc::new(Node::Literal(Token::Integer(100))),
        ));
        println!("obj: {:?}", v1);
        assert_eq!(v1.is_ok(), true);
        assert_eq!(Object::Number(-99 as f64), v1.unwrap());

        let v1 = intpter.eval(Node::Binary(
            Rc::new(Node::Literal(Token::Integer(2))),
            Token::Star('*'),
            Rc::new(Node::Literal(Token::Integer(256))),
        ));
        println!("obj: {:?}", v1);
        assert_eq!(v1.is_ok(), true);
        assert_eq!(Object::Number(512 as f64), v1.unwrap());

        let v1 = intpter.eval(Node::Binary(
            Rc::new(Node::Literal(Token::Integer(2048))),
            Token::Slash('/'),
            Rc::new(Node::Literal(Token::Integer(2))),
        ));
        println!("2048 / 2 = : {:?}", v1);
        assert_eq!(v1.is_ok(), true);
        assert_eq!(Object::Number(1024 as f64), v1.unwrap());
    }

    #[test]
    fn test_eval_compare() {
        let nodes = vec![
            Node::Binary(
                Rc::new(Node::Literal(Token::Integer(1024))),
                Token::Lt("<".to_string()),
                Rc::new(Node::Literal(Token::Integer(1024))),
            ),
            Node::Binary(
                Rc::new(Node::Literal(Token::Integer(1000))),
                Token::LtEQ("<=".to_string()),
                Rc::new(Node::Literal(Token::Integer(1024))),
            ),
            Node::Binary(
                Rc::new(Node::Literal(Token::SString("abc".to_string()))),
                Token::Gt(">".to_string()),
                Rc::new(Node::Literal(Token::SString("xyz".to_string()))),
            ),
            Node::Binary(
                Rc::new(Node::Literal(Token::SString("def".to_string()))),
                Token::GtEQ(">=".to_string()),
                Rc::new(Node::Literal(Token::SString("abc".to_string()))),
            ),
        ];

        let mut intpter = Interpreter::new();
        let expect_result = vec![
            Object::Bool(false),
            Object::Bool(true),
            Object::Bool(false),
            Object::Bool(true),
        ];

        let mut idx = 0;
        for n in nodes {
            let v = intpter.eval(n);
            println!("obj: {:?}", v);
            assert_eq!(v.is_ok(), true);
            assert_eq!(expect_result[idx], v.unwrap());
            idx += 1;
        }
    }

    #[test]
    fn test_eval_compare_err() {
        let n = Node::Binary(
            Rc::new(Node::Literal(Token::SString("def".to_string()))),
            Token::GtEQ(">=".to_string()),
            Rc::new(Node::Literal(Token::Integer(1024))),
        );
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        println!("obj: {:?}", v);
        assert_eq!(v.is_err(), true);
        assert_eq!(
            v.err(),
            Some(EvalError::DifferObjectToCompare(
                Object::SString("def".to_string()),
                Object::Integer(1024)
            ))
        );
    }

    #[test]
    fn test_eval_var_stmt() {
        let n = Node::VarStmt(ast::VarStmt::new(
            Token::Ident("x".to_string()),
            Node::Literal(Token::Integer(1024)),
        ));
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        println!("obj: {:?}", v);
        assert_eq!(v.is_ok(), true);
        assert_eq!(Object::Integer(1024), v.unwrap());

        let x_v = intpter.get_value("x".to_string());
        println!("x_v: {:?}", x_v);
        assert_eq!(Some(&Object::Integer(1024)), x_v);
    }

    #[test]
    fn test_eval_var_stmt1() {
        let n = Node::VarStmt(ast::VarStmt::new(
            Token::Integer(1),
            Node::Literal(Token::Integer(1024)),
        ));
        let mut intpter = Interpreter::new();
        let v = intpter.eval(n);
        println!("obj: {:?}", v);
        assert_eq!(v.is_err(), true);
        assert_eq!(
            Some(EvalError::NotIdent(Token::SString("x".to_string()))),
            v.err()
        );
    }
}
