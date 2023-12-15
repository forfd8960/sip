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
        if self.match_tks(vec![TokenType::Var]) {
            return self.parse_var();
        }

        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn parse_var(&mut self) -> Result<Node, ParserError> {
        let ident_res = self.consume(TokenType::Ident, "expect ident token".to_string())?;

        if self.match_tks(vec![TokenType::Assign]) {}

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
        if self.match_tks(vec![TokenType::Assign]) {
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
        if self.match_tks(vec![TokenType::Or]) {
            return Err(ParserError::NotSupportedToken(Token::Unkown));
        }

        Ok(res)
    }

    fn and(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn equality(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn comparison(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn term(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn factor(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
    }

    fn unary(&mut self) -> Result<Node, ParserError> {
        Err(ParserError::NotSupportedToken(Token::Unkown))
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
