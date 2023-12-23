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

        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Node, ParserError> {
        let res = self.assignment()?;
        Ok(res)
    }

    fn parse_if(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::LParent, "expect ( after if".to_string())?;

        let cond = self.parse_expr()?;
        self.consume(TokenType::RParent, "expect ) after if".to_string())?;

        let then = self.parse_stmt()?;

        let mut else_then = Node::Null;
        if self.match_tk(TokenType::Else) {
            else_then = self.parse_stmt()?;
        }

        Ok(Node::IfStmt(
            Rc::new(cond),
            Rc::new(then),
            Rc::new(else_then),
        ))
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
        let mut res = self.and()?;
        loop {
            if self.match_tk(TokenType::Or) {
                let op = self.previous();
                let exp = self.and()?;

                res = Node::Logical(Rc::new(res), op, Rc::new(exp));
                continue;
            }
            break;
        }

        Ok(res)
    }

    fn and(&mut self) -> Result<Node, ParserError> {
        let mut exp = self.equality()?;
        loop {
            if self.match_tk(TokenType::And) {
                let op = self.previous();
                let r_exp = self.equality()?;
                exp = Node::Logical(Rc::new(exp), op, Rc::new(r_exp));
                continue;
            }

            break;
        }
        Ok(exp)
    }

    fn equality(&mut self) -> Result<Node, ParserError> {
        let mut exp = self.comparison()?;
        loop {
            if self.match_tks(vec![TokenType::EQ, TokenType::NotEQ]) {
                let op = self.previous();
                let r_exp = self.comparison()?;
                exp = Node::Binary(Rc::new(exp), op, Rc::new(r_exp));
                continue;
            }

            break;
        }
        Ok(exp)
    }

    fn comparison(&mut self) -> Result<Node, ParserError> {
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
                exp = Node::Binary(Rc::new(exp), op, Rc::new(r_exp));
                continue;
            }
            break;
        }
        Ok(exp)
    }

    fn term(&mut self) -> Result<Node, ParserError> {
        let mut exp = self.factor()?;
        loop {
            if self.match_tks(vec![TokenType::Plus, TokenType::Minus]) {
                let op = self.previous();
                let r_exp = self.factor()?;
                exp = Node::Binary(Rc::new(exp), op, Rc::new(r_exp));
            } else {
                break;
            }
        }

        Ok(exp)
    }

    fn factor(&mut self) -> Result<Node, ParserError> {
        let mut exp = self.unary()?;
        loop {
            if self.match_tks(vec![TokenType::Slash, TokenType::Star]) {
                let op = self.previous();
                let r_exp = self.unary()?;
                exp = Node::Binary(Rc::new(exp), op, Rc::new(r_exp));
                continue;
            }

            break;
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

    #[test]
    fn test_parse_literal() {
        let mut parser = Parser::new(vec![
            Token::Integer(100),
            Token::Float(66.88),
            Token::SString("Hello".to_string()),
            Token::True,
            Token::False,
            Token::EOF,
        ]);
        let res = parser.parse();
        println!("parse result: {:?}", res);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            Program::new(vec![
                Node::Literal(Token::Integer(100)),
                Node::Literal(Token::Float(66.88)),
                Node::Literal(Token::SString("Hello".to_string())),
                Node::Literal(Token::True),
                Node::Literal(Token::False),
            ]),
            res.unwrap()
        );
    }

    #[test]
    fn test_parse_binary() {
        let mut parser = Parser::new(vec![
            Token::Integer(100),
            Token::Plus('+'),
            Token::Integer(1000),
            Token::Star('*'),
            Token::Integer(2),
            Token::EOF,
        ]);
        let res = parser.parse();
        println!("parse result: {:?}", res);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            Program::new(vec![Node::Binary(
                Rc::new(Node::Literal(Token::Integer(100))),
                Token::Plus('+'),
                Rc::new(Node::Binary(
                    Rc::new(Node::Literal(Token::Integer(1000))),
                    Token::Star('*'),
                    Rc::new(Node::Literal(Token::Integer(2)))
                ))
            )]),
            res.unwrap()
        );
    }

    #[test]
    fn test_parse_and_or() {
        let mut parser = Parser::new(vec![
            Token::Ident("a".to_string()),
            Token::Lt("<".to_string()),
            Token::Ident("b".to_string()),
            Token::And,
            Token::Ident("b".to_string()),
            Token::Gt(">".to_string()),
            Token::Ident("c".to_string()),
            Token::EOF,
        ]);
        let res = parser.parse();
        println!("parse result: {:?}", res);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            Program::new(vec![Node::Logical(
                Rc::new(Node::Binary(
                    Rc::new(Node::Identifier(Token::Ident("a".to_string()))),
                    Token::Lt("<".to_string()),
                    Rc::new(Node::Identifier(Token::Ident("b".to_string()))),
                )),
                Token::And,
                Rc::new(Node::Binary(
                    Rc::new(Node::Identifier(Token::Ident("b".to_string()))),
                    Token::Gt(">".to_string()),
                    Rc::new(Node::Identifier(Token::Ident("c".to_string()))),
                )),
            )]),
            res.unwrap()
        );
    }

    #[test]
    fn test_parse_if() {
        let mut parser = Parser::new(vec![
            Token::If,
            Token::LParent('('),
            Token::Ident("b".to_string()),
            Token::Gt(">".to_string()),
            Token::Ident("a".to_string()),
            Token::RParent(')'),
            Token::LBrace('{'),
            Token::SString("then stmt".to_string()),
            Token::RBrace('}'),
            Token::EOF,
        ]);
        let res = parser.parse();
        println!("parse result: {:?}", res);
        assert_eq!(res.is_ok(), true);
        assert_eq!(
            Program::new(vec![Node::IfStmt(
                Rc::new(Node::Binary(
                    Rc::new(Node::Identifier(Token::Ident("b".to_string()))),
                    Token::Gt(">".to_string()),
                    Rc::new(Node::Identifier(Token::Ident("a".to_string()))),
                )),
                Rc::new(Node::Block(vec![Node::Literal(Token::SString(
                    "then stmt".to_string()
                ))])),
                Rc::new(Node::Null)
            )]),
            res.unwrap()
        );
    }
}
