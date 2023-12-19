use crate::{
    ast::{Node, Program},
    errors::EvalError,
    object::Object,
    tokens::Token,
};

pub fn eval_program(pgram: Program) -> Result<Object, EvalError> {
    let mut r = Object::Null;
    for node in pgram.stmts {
        r = eval(node)?;
    }
    Ok(r)
}

pub fn eval(node: Node) -> Result<Object, EvalError> {
    match node {
        Node::Literal(tk) => eval_literal(tk),
        _ => Err(EvalError::UnknowNode(node)),
    }
}

fn eval_literal(tk: Token) -> Result<Object, EvalError> {
    match tk {
        Token::Integer(v) => Ok(Object::Integer(v)),
        Token::Float(v) => Ok(Object::Float(v)),
        Token::SString(v) => Ok(Object::SString(v)),
        Token::True => Ok(Object::Bool(true)),
        Token::False => Ok(Object::Bool(false)),
        _ => Err(EvalError::NotLiteral(tk)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Node, object::Object, tokens::Token};

    use super::eval;

    #[test]
    fn test_eval_integer() {
        let n = Node::Literal(Token::Integer(100));
        let v = eval(n);
        assert_eq!(v.is_ok(), true);
        assert_eq!(Object::Integer(100), v.unwrap());
    }

    #[test]
    fn test_eval_literal1() {
        let n = Node::Literal(Token::SString("Hello, World!".to_string()));
        let v = eval(n);
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
        let mut idx = 0 as usize;
        for node in nodes {
            let v = eval(node);
            assert_eq!(v.is_ok(), true);
            assert_eq!(expect_obj[idx], v.unwrap());
            idx += 1;
        }
    }
}
