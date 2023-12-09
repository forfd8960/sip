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
            if self.is_at_end() {
                tokens.push(Token::EOF);
                break;
            }

            self.start = self.current;
            let tk_result = self.next_token();
            match tk_result {
                Ok(Token::WhiteSpace) => {}
                Ok(tk) => {
                    tokens.push(tk.clone());
                }

                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let ch = self.advance();
        println!("ch: {}", ch);

        match ch {
            ' ' | '\n' | '\t' | '\r' => Ok(Token::WhiteSpace),
            '(' => Ok(Token::LParent('(')),
            ')' => Ok(Token::RParent(')')),
            '{' => Ok(Token::LBrace('{')),
            '}' => Ok(Token::RBrace('}')),
            '[' => Ok(Token::LSBracket('[')),
            ']' => Ok(Token::RSBracket(']')),
            '+' => Ok(Token::Plus(ch)),
            '-' => Ok(Token::Minus(ch)),
            '*' => Ok(Token::Star(ch)),
            '/' => Ok(Token::Slash(ch)),
            '=' => {
                if self.is_current_match('=') {
                    Ok(Token::EQ("==".to_string()))
                } else {
                    Ok(Token::Assign(ch))
                }
            }
            '<' => {
                if self.is_current_match('=') {
                    Ok(Token::LtEQ("<=".to_string()))
                } else {
                    Ok(Token::Lt("<".to_string()))
                }
            }
            '>' => {
                if self.is_current_match('=') {
                    Ok(Token::GtEQ(">=".to_string()))
                } else {
                    Ok(Token::Gt(">".to_string()))
                }
            }
            _ => {
                if ch.is_digit(10) {
                    let n = self.parse_number()?;
                    return Ok(Token::Integer(n));
                }

                Err(LexerError::InvalidToken(ch))
            }
        }
    }

    fn is_current_match(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.chars[self.current] != ch {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        Some(self.chars[self.current])
    }

    fn parse_number(&mut self) -> Result<i64, LexerError> {
        while let Some(d) = self.peek() {
            if d.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        let num_text = String::from_iter(&self.chars[self.start..self.current]);
        println!("num_text: {}", num_text);

        let num_res = num_text.parse::<i64>();
        match num_res {
            Ok(n) => Ok(n),
            Err(e) => Err(LexerError::InvalidNum(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::tokens::{self, Token};

    #[test]
    fn test_scan_tokens1() {
        let input = " \n + - * \t /";
        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::Plus('+'),
                Token::Minus('-'),
                Token::Star('*'),
                Token::Slash('/'),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_tokens2() {
        let input = "( { [ ) } ]";
        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::LParent('('),
                Token::LBrace('{'),
                Token::LSBracket('['),
                Token::RParent(')'),
                Token::RBrace('}'),
                Token::RSBracket(']'),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_tokens3() {
        let input = "< <= > >=";
        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::Lt('<'.to_string()),
                Token::LtEQ("<=".to_string()),
                Token::Gt(">".to_string()),
                Token::GtEQ(">=".to_string()),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_tokens4() {
        let input = "100 256";
        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        assert_eq!(tokens_res.is_ok(), true);
        println!("{:?}", tokens_res);
        assert_eq!(
            vec![
                Token::Integer(100 as i64),
                Token::Integer(256 as i64),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }
}
