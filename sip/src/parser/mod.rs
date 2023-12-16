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

        self.parse_stmt()
    }

    fn parse_var(&mut self) -> Result<Node, ParserError> {
        let ident = self.consume(TokenType::Ident, "expect ident token".to_string())?;

        let mut init_expr = Node::Null;
        if self.match_tk(TokenType::Assign) {
            init_expr = self.parse_expr()?;
        }

        Ok(Node::VarStmt(ident, Rc::new(init_expr)))
    }

    fn parse_stmt(&mut self) -> Result<Node, ParserError> {
        if self.match_tk(TokenType::If) {
            return self.parse_if();
        }
        if self.match_tk(TokenType::For) {
            return self.parse_for();
        }

        if self.match_tk(TokenType::While) {
            return self.parse_while();
        }

        if self.match_tk(TokenType::Return) {
            return self.parse_return();
        }

        if self.match_tk(TokenType::LBrace) {
            return self.parse_block();
        }

        self.parse_expr_stmt()
    }

    fn parse_expr(&mut self) -> Result<Node, ParserError> {
        let res = self.assignment()?;
        Ok(res)
    }

    fn parse_if(&mut self) -> Result<Node, ParserError> {
        Ok(Node::Null)
    }

    fn parse_for(&mut self) -> Result<Node, ParserError> {
        Ok(Node::Null)
    }

    fn parse_while(&mut self) -> Result<Node, ParserError> {
        Ok(Node::Null)
    }

    fn parse_return(&mut self) -> Result<Node, ParserError> {
        Ok(Node::Null)
    }

    fn parse_block(&mut self) -> Result<Node, ParserError> {
        let mut stmts: Vec<Node> = vec![];
        loop {
            if self.is_at_end() || self.check(TokenType::RBrace) {
                break;
            }

            let stmt = self.declare()?;
            stmts.push(stmt);
        }

        self.consume(TokenType::RBrace, "expect } after block".to_string())?;

        Ok(Node::Block(stmts))
    }

    fn parse_expr_stmt(&mut self) -> Result<Node, ParserError> {
        let exp = self.parse_expr()?;
        Ok(Node::ExpressionStmt(Rc::new(exp)))
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
            let op = self.previous();
            let exp = self.and()?;

            return Ok(Node::Logical(Rc::new(res), op, Rc::new(exp)));
        }

        Ok(res)
    }

    fn and(&mut self) -> Result<Node, ParserError> {
        let exp = self.equality()?;
        if self.match_tk(TokenType::And) {
            let op = self.previous();
            let r_exp = self.equality()?;
            return Ok(Node::Logical(Rc::new(exp), op, Rc::new(r_exp)));
        }
        Ok(exp)
    }

    fn equality(&mut self) -> Result<Node, ParserError> {
        let exp = self.comparison()?;
        if self.match_tks(vec![TokenType::EQ, TokenType::NotEQ]) {
            let op = self.previous();
            let r_exp = self.comparison()?;
            return Ok(Node::Binary(Rc::new(exp), op, Rc::new(r_exp)));
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
            let op = self.previous();
            let r_exp = self.term()?;
            return Ok(Node::Binary(Rc::new(exp), op, Rc::new(r_exp)));
        }
        Ok(exp)
    }

    fn term(&mut self) -> Result<Node, ParserError> {
        let exp = self.factor()?;
        if self.match_tks(vec![TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let r_exp = self.factor()?;
            return Ok(Node::Binary(Rc::new(exp), op, Rc::new(r_exp)));
        }
        Ok(exp)
    }

    fn factor(&mut self) -> Result<Node, ParserError> {
        let exp = self.unary()?;
        if self.match_tks(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let r_exp = self.unary()?;
            return Ok(Node::Binary(Rc::new(exp), op, Rc::new(r_exp)));
        }

        Ok(exp)
    }

    fn unary(&mut self) -> Result<Node, ParserError> {
        if self.match_tks(vec![TokenType::Minus, TokenType::Bang]) {
            let op = self.previous();
            let val = self.unary()?;
            return Ok(Node::Unary(op, Rc::new(val)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Node, ParserError> {
        if self.match_tk(TokenType::True)
            || self.match_tk(TokenType::False)
            || self.match_tk(TokenType::Integer)
            || self.match_tk(TokenType::Float)
            || self.match_tk(TokenType::String)
            || self.match_tk(TokenType::Null)
        {
            return Ok(Node::Literal(self.previous()));
        } else if self.match_tk(TokenType::Ident) {
            return Ok(Node::Identifier(self.previous()));
        } else if self.match_tk(TokenType::LParent) {
            let exp = self.parse_expr()?;
            return Ok(Node::Group(Rc::new(exp)));
        }

        Err(ParserError::NotSupportedToken(self.peek()))
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
