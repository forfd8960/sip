use crate::{
    errors::LexerError,
    tokens::{self, Token},
};

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
        println!("next token: ch: {}", ch);

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
            '!' => Ok(Token::Bang),
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
            '|' => {
                if self.is_current_match('|') {
                    Ok(Token::Or)
                } else {
                    Ok(Token::BitOr)
                }
            }
            '&' => {
                if self.is_current_match('&') {
                    Ok(Token::And)
                } else {
                    Ok(Token::BitAnd)
                }
            }
            '"' => self.parse_string(),
            _ => {
                if ch.is_digit(10) {
                    let tk = self.parse_number()?;
                    return Ok(tk);
                }

                if ch.is_alphabetic() {
                    let tk = self.parse_ident()?;
                    return Ok(tk);
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

    fn parse_number(&mut self) -> Result<Token, LexerError> {
        let mut is_float = false;
        while let Some(d) = self.peek() {
            if d.eq(&'.') || d.is_digit(10) {
                self.advance();
                if d.eq(&'.') {
                    is_float = true;
                }
            } else {
                break;
            }
        }

        let num_text = String::from_iter(&self.chars[self.start..self.current]);

        if is_float {
            let num_res = num_text.parse::<f64>();
            let res = match num_res {
                Ok(n) => Ok(Token::Float(n)),
                Err(e) => Err(LexerError::InvalidNum(e.to_string())),
            };
            return res;
        }

        let num_res = num_text.parse::<i64>();
        match num_res {
            Ok(n) => Ok(Token::Integer(n)),
            Err(e) => Err(LexerError::InvalidNum(e.to_string())),
        }
    }

    fn parse_string(&mut self) -> Result<Token, LexerError> {
        let mut valid_str = false;
        while let Some(c) = self.peek() {
            self.advance();
            if c.eq(&'"') {
                valid_str = true;
                break;
            }
        }

        if self.is_at_end() && !valid_str {
            let str_content = String::from_iter(&self.chars[self.start..]);
            return Err(LexerError::InvalidString(str_content));
        }

        let str_content = String::from_iter(&self.chars[self.start + 1..self.current - 1]);
        Ok(Token::SString(str_content))
    }

    fn parse_ident(&mut self) -> Result<Token, LexerError> {
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            self.advance();
        }

        let ident = String::from_iter(&self.chars[self.start..self.current]);
        if let Some(tk) = tokens::get_keyword(ident.as_str()) {
            return Ok(tk);
        }

        Ok(Token::Ident(ident))
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::tokens::Token;

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

    #[test]
    fn test_scan_string_tokens() {
        let input = r#""Hello" "World""#;

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();

        assert_eq!(tokens_res.is_ok(), true);
        println!("{:?}", tokens_res);
        assert_eq!(
            vec![
                Token::SString("Hello".to_string()),
                Token::SString("World".to_string()),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_string1_tokens() {
        let input = r#""Hello" "World" 866"#;
        println!("input: {:?}", input);

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        println!("{:?}", tokens_res);

        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::SString("Hello".to_string()),
                Token::SString("World".to_string()),
                Token::Integer(866 as i64),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_ident_tokens() {
        let input = "abc func1";

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        println!("{:?}", tokens_res);

        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::Ident("abc".to_string()),
                Token::Ident("func1".to_string()),
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_keyword_tokens() {
        let input = "true false var print";

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        println!("{:?}", tokens_res);

        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::True,
                Token::False,
                Token::Var,
                Token::Print("print".to_string()),
                Token::EOF
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_keyword_tokens1() {
        let input = "if else for while return";

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        println!("{:?}", tokens_res);

        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::If,
                Token::Else,
                Token::For,
                Token::While,
                Token::Return,
                Token::EOF,
            ],
            tokens_res.unwrap()
        );
    }

    #[test]
    fn test_scan_float_num() {
        let input = "1.26 0.6 123";

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        println!("{:?}", tokens_res);

        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::Float(1.26 as f64),
                Token::Float(0.6 as f64),
                Token::Integer(123 as i64),
                Token::EOF,
            ],
            tokens_res.unwrap()
        )
    }

    #[test]
    fn test_scan_or_and() {
        let input = "| || & &&";

        let mut lexer = Lexer::new(input.to_string());
        let tokens_res = lexer.scan_tokens();
        println!("{:?}", tokens_res);

        assert_eq!(tokens_res.is_ok(), true);
        assert_eq!(
            vec![
                Token::BitOr,
                Token::Or,
                Token::BitAnd,
                Token::And,
                Token::EOF
            ],
            tokens_res.unwrap()
        )
    }

    #[test]
    fn test_parse_float() {
        let num_text = "1.266";
        match num_text.parse::<f64>() {
            Ok(n) => println!("num: {}", n),
            Err(e) => println!("err: {}", e),
        }
    }
}
