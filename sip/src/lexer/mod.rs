use crate::tokens::Token;

pub struct Lexer {
    text: String,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            start: 0 as usize,
            current: 0 as usize,
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
