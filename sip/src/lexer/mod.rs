use crate::{errors::LexerError, tokens::Token};

pub struct Lexer {
    text: String,
    chars: Vec<char>,
    start: usize,
    current: usize,
}

fn is_white_space(ch: char) -> bool {
    match ch {
        '\n' | '\t' | '\r' | ' ' => true,
        _ => false,
    }
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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let tk_result = self.next_token();
            match tk_result {
                Ok(tk) => {
                    tokens.push(tk.clone());
                    if tk == Token::EOF {
                        break;
                    }
                }

                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let (ch, is_wsp) = self.consume_white_space();
        if is_wsp && self.is_at_end() {
            return Ok(Token::EOF);
        }

        match ch {
            '=' => Ok(Token::Assign("=".to_string())),
            _ => Err(LexerError::InvalidToken(ch)),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn consume_white_space(&mut self) -> (char, bool) {
        let mut ch = self.advance();
        let mut white_space = false;
        loop {
            if !is_white_space(ch) {
                break;
            }
            white_space = true;

            ch = self.advance();
            if self.is_at_end() {
                break;
            }
        }

        if white_space {
            self.start = self.current - 1;
        }

        (ch, white_space)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }
}
