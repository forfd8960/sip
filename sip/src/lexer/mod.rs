use crate::{errors::LexerError, tokens::Token};

pub struct Lexer {
    text: String,
    chars: Vec<char>,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let text_clone = text.clone();
        Self {
            text: text,
            start: 0 as usize,
            current: 0 as usize,
            chars: text_clone.chars().collect(),
        }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, LexerError> {
        Ok(vec![])
    }
}
