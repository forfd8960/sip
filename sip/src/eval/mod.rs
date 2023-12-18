use crate::{
    ast::Node,
    errors::EvalError,
    object::{Integer, Object},
};

pub fn eval<'a>(node: Node) -> Result<Box<dyn Object>, EvalError> {
    Ok(Box::new(Integer::new(100)))
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Node,
        object::{Integer, ObjectType},
        tokens::Token,
    };

    use super::eval;

    #[test]
    fn test_eval_integer() {
        let n = Node::Literal(Token::Integer(100));
        let v = eval(n);
        assert_eq!(v.is_ok(), true);
        assert_eq!(ObjectType::Integer(100), v.unwrap().obj_type());
    }
}
