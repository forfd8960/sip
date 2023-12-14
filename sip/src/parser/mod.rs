use std::rc::Rc;

use crate::{ast::Node, ast::Program, errors::ParserError, tokens::Token};

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
        if self.check_and_advance(vec![Token::Var]) {
            return self.parse_var();
        }

        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn parse_var(&mut self) -> Result<Node, ParserError> {
        let ident_res = self.consume(
            Token::Ident("".to_string()),
            "expect ident token".to_string(),
        )?;

        if self.check_and_advance(vec![Token::Assign('=')]) {}

        Ok(Node::VarStmt(
            Token::Var,
            Rc::new(Node::Literal(Token::Integer(100))),
        ))
    }

    fn parse_expr(&mut self) -> Result<Node, ParserError> {
        let res = self.assignment()?;
        Ok(res)
    }

    fn assignment(&mut self) -> Result<Node, ParserError> {
        let exp = self.or()?;
        if self.check_and_advance(vec![Token::Assign('=')]) {
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
        if self.check_and_advance(vec![Token::Or]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }

        Ok(res)
    }

    fn and(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn check_and_advance(&mut self, tokens: Vec<Token>) -> bool {
        for tk in tokens {
            if self.check(tk) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, tk: Token, msg: String) -> Result<Token, ParserError> {
        if self.check(tk) {
            return Ok(self.advance());
        }

        Err(ParserError::ExpectedTokenNotFound(msg))
    }

    fn check(&mut self, token: Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == token.token_type()
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
