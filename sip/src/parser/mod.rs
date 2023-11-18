use crate::{errors::ParserError, tokens::Token};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            start: 0 as usize,
            current: 0 as usize,
        }
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        Ok(())
    }
}
