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

        Err(ParserError::NotSupportedToken(Token::SString(
            "".to_string(),
        )))
    }

    fn parse_var(&mut self) -> Result<Node, ParserError> {
        Ok(Node::VarStmt(
            Token::Var,
            Rc::new(Node::Literal(Token::Integer(100))),
        ))
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

    fn check(&mut self, token: Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek() == token
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
