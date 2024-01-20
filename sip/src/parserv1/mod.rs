use crate::{
    astv1::{self, Node, NodeType, Program},
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
        let mut nodes: Vec<Box<dyn Node>> = vec![];
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

    fn declare(&mut self) -> Result<Box<dyn Node>, ParserError> {
        if self.match_tk(TokenType::Var) {
            return self.parse_var();
        }

        self.parse_stmt()
    }

    fn parse_var(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let ident = self.consume(TokenType::Ident, "expect ident token".to_string())?;

        let mut init_expr: Box<dyn Node> = Box::new(astv1::Null::new());
        if self.match_tk(TokenType::Assign) {
            init_expr = self.parse_expr()?;
        }

        Ok(Box::new(astv1::VarStmt::new(ident, init_expr)))
    }

    fn parse_stmt(&mut self) -> Result<Box<dyn Node>, ParserError> {
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

        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let res = self.assignment()?;
        Ok(res)
    }

    fn parse_if(&mut self) -> Result<Box<dyn Node>, ParserError> {
        self.consume(TokenType::LParent, "expect ( after if".to_string())?;

        let cond = self.parse_expr()?;
        self.consume(TokenType::RParent, "expect ) after if".to_string())?;

        let then = self.parse_stmt()?;

        let mut else_then: Box<dyn Node> = Box::new(astv1::Null::new());
        if self.match_tk(TokenType::Else) {
            else_then = self.parse_stmt()?;
        }

        Ok(Box::new(astv1::IfStmt::new(cond, then, else_then)))
    }

    fn parse_for(&mut self) -> Result<Box<dyn Node>, ParserError> {
        todo!()
    }

    fn parse_while(&mut self) -> Result<Box<dyn Node>, ParserError> {
        todo!()
    }

    fn parse_return(&mut self) -> Result<Box<dyn Node>, ParserError> {
        todo!()
    }

    fn parse_block(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut stmts: Vec<Box<dyn Node>> = vec![];
        loop {
            if self.is_at_end() || self.check(TokenType::RBrace) {
                break;
            }

            let stmt = self.declare()?;
            stmts.push(stmt);
        }

        self.consume(TokenType::RBrace, "expect } after block".to_string())?;

        Ok(Box::new(astv1::Block::new(stmts)))
    }

    fn assignment(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let exp = self.or()?;
        if self.match_tk(TokenType::Assign) {
            let value = self.assignment()?;
            let res: Result<Box<dyn Node>, ParserError> = match exp.typ() {
                NodeType::Identifier => Ok(Box::new(astv1::Assign::new(
                    exp.as_ident().unwrap().name,
                    value,
                ))),
                _ => Err(ParserError::NotSupportedToken(Token::Unkown)),
            };
            return res;
        }

        Ok(exp)
    }

    fn or(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut res = self.and()?;
        loop {
            if self.match_tk(TokenType::Or) {
                let op = self.previous();
                let exp = self.and()?;

                res = Box::new(astv1::Logical::new(res, op, exp));
                continue;
            }
            break;
        }

        Ok(res)
    }

    fn and(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut exp = self.equality()?;
        loop {
            if self.match_tk(TokenType::And) {
                let op = self.previous();
                let r_exp = self.equality()?;
                exp = Box::new(astv1::Logical::new(exp, op, r_exp));
                continue;
            }

            break;
        }
        Ok(exp)
    }

    fn equality(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut left = self.comparison()?;
        loop {
            if self.match_tks(vec![TokenType::EQ, TokenType::NotEQ]) {
                let op = self.previous();
                let right = self.comparison()?;
                left = Box::new(astv1::Binary::new(left, op, right));
                continue;
            }

            break;
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut exp = self.term()?;
        loop {
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
                exp = Box::new(astv1::Binary::new(exp, op, r_exp));
                continue;
            }
            break;
        }
        Ok(exp)
    }

    fn term(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut exp = self.factor()?;
        loop {
            if self.match_tks(vec![TokenType::Plus, TokenType::Minus]) {
                let op = self.previous();
                let r_exp = self.factor()?;
                exp = Box::new(astv1::Binary::new(exp, op, r_exp));
            } else {
                break;
            }
        }

        Ok(exp)
    }

    fn factor(&mut self) -> Result<Box<dyn Node>, ParserError> {
        let mut exp = self.unary()?;
        loop {
            if self.match_tks(vec![TokenType::Slash, TokenType::Star]) {
                let op = self.previous();
                let r_exp = self.unary()?;
                exp = Box::new(astv1::Binary::new(exp, op, r_exp));
                continue;
            }

            break;
        }

        Ok(exp)
    }

    fn unary(&mut self) -> Result<Box<dyn Node>, ParserError> {
        if self.match_tks(vec![TokenType::Minus, TokenType::Bang]) {
            let op = self.previous();
            let val = self.unary()?;
            return Ok(Box::new(astv1::Unary::new(op, val)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<dyn Node>, ParserError> {
        if self.match_tk(TokenType::True)
            || self.match_tk(TokenType::False)
            || self.match_tk(TokenType::Integer)
            || self.match_tk(TokenType::Float)
            || self.match_tk(TokenType::String)
            || self.match_tk(TokenType::Null)
        {
            return Ok(Box::new(astv1::Literal::new(self.previous())));
        } else if self.match_tk(TokenType::Ident) {
            return Ok(Box::new(astv1::Ident::new(self.previous())));
        } else if self.match_tk(TokenType::LParent) {
            let exp = self.parse_expr()?;
            return Ok(Box::new(astv1::Group::new(exp)));
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
