use super::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token<T: Clone> {
    pub token_type: TokenType,
    pub literal: String,
    pub value: T,
}

impl<T: Clone> Token<T> {
    pub fn new(tp: TokenType, liternal: String, value: T) -> Self {
        Self {
            token_type: tp,
            literal: liternal,
            value: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::TokenType;

    use super::Token;

    #[test]
    fn test_new_token() {
        let s_token = Token::new(TokenType::String, "good".to_string(), "good".to_string());
        println!("{:?}", s_token);

        let s_token = Token::new(TokenType::Integer, 100.to_string(), 100);
        println!("{:?}", s_token);

        let s_token = Token::new(TokenType::True, true.to_string(), true);
        println!("{:?}", s_token);
    }
}
