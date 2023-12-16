use std::rc::Rc;

use crate::{
    ast::Node,
    ast::Program,
    errors::ParserError,
    tokens::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0 as usize,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut nodes: Vec<Node> = vec![];
        loop {
            if self.is_at_end() {
                break;
            }

            let node_res = self.declare();
            match node_res {
                Ok(v) => {
                    nodes.push(v);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(Program::new(nodes))
    }

    fn declare(&mut self) -> Result<Node, ParserError> {
        if self.match_tk(TokenType::Var) {
            return self.parse_var();
        }

        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn parse_var(&mut self) -> Result<Node, ParserError> {
        let ident = self.consume(TokenType::Ident, "expect ident token".to_string())?;

        let mut init_expr = Node::Null;
        if self.match_tk(TokenType::Assign) {
            init_expr = self.parse_expr()?;
        }

        Ok(Node::VarStmt(ident, Rc::new(init_expr)))
    }

    fn parse_expr(&mut self) -> Result<Node, ParserError> {
        let res = self.assignment()?;
        Ok(res)
    }

    fn assignment(&mut self) -> Result<Node, ParserError> {
        let exp = self.or()?;
        if self.match_tk(TokenType::Assign) {
            let value = self.assignment()?;
            let res = match exp {
                Node::Identifier(ident) => Ok(Node::Assign(ident, Rc::new(value))),
                _ => Err(ParserError::NotSupportedToken(Token::Unkown)),
            };
            return res;
        }

        Ok(exp)
    }

    fn or(&mut self) -> Result<Node, ParserError> {
        let res = self.and()?;
        if self.match_tk(TokenType::Or) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }

        Ok(res)
    }

    fn and(&mut self) -> Result<Node, ParserError> {
        let exp = self.equality()?;
        if self.match_tk(TokenType::And) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }
        Ok(exp)
    }

    fn equality(&mut self) -> Result<Node, ParserError> {
        let exp = self.comparison()?;
        if self.match_tks(vec![TokenType::EQ, TokenType::NotEQ]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }
        Ok(exp)
    }

    fn comparison(&mut self) -> Result<Node, ParserError> {
        let exp = self.term()?;
        if self.match_tks(vec![
            TokenType::Lt,
            TokenType::LtEQ,
            TokenType::Gt,
            TokenType::GtEQ,
            TokenType::EQ,
            TokenType::NotEQ,
        ]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }
        Ok(exp)
    }

    fn term(&mut self) -> Result<Node, ParserError> {
        let exp = self.factor()?;
        if self.match_tks(vec![TokenType::Plus, TokenType::Minus]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }
        Ok(exp)
    }

    fn factor(&mut self) -> Result<Node, ParserError> {
        let exp = self.unary()?;
        if self.match_tks(vec![TokenType::Slash, TokenType::Star]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }

        Ok(exp)
    }

    fn unary(&mut self) -> Result<Node, ParserError> {
        if self.match_tks(vec![TokenType::Minus, TokenType::Bang]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Node, ParserError> {
        if self.match_tk(TokenType::True)
            || self.match_tk(TokenType::False)
            || self.match_tk(TokenType::Integer)
            || self.match_tk(TokenType::Float)
            || self.match_tk(TokenType::Null)
        {
            return Ok(Node::Literal(self.previous()));
        } else if self.match_tk(TokenType::Ident) {
            return Ok(Node::Identifier(self.previous()));
        } else if self.match_tk(TokenType::LParent) {
            let exp = self.parse_expr()?;
            return Ok(Node::Group(Rc::new(exp)));
        }

        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn match_tk(&mut self, tk_type: TokenType) -> bool {
        self.match_tks(vec![tk_type])
    }

    fn match_tks(&mut self, tk_types: Vec<TokenType>) -> bool {
        for tk_tp in tk_types {
            if self.check(tk_tp) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, tk_type: TokenType, msg: String) -> Result<Token, ParserError> {
        if self.check(tk_type) {
            return Ok(self.advance());
        }

        Err(ParserError::ExpectedTokenNotFound(msg))
    }

    fn check(&mut self, tk_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == tk_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Token::EOF
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{ast::Node, ast::Program, tokens::Token};
    use std::rc::Rc;

    #[test]
    fn test_parse_var() {
        let mut parser = Parser::new(vec![
            Token::Var,
            Token::Ident("x".to_string()),
            Token::Assign('='),
            Token::Integer(100),
            Token::EOF,
        ]);
        let res = parser.parse();
        println!("parse result: {:?}", res);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            Program::new(vec![Node::VarStmt(
                Token::Ident("x".to_string()),
                Rc::new(Node::Literal(Token::Integer(100)))
            )]),
            res.unwrap()
        );
    }
}
