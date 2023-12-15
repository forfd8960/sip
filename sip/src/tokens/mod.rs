#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    WhiteSpace,
    Ident(String),
    Integer(i64),
    Float(f64),
    SString(String),
    True,
    False,

    Var,           // keyword: var
    Print(String), // keyword: print
    If,
    Else,
    For,
    While,
    Return,

    Assign(char), // =
    Plus(char),   // +
    Minus(char),  // -
    Star(char),   // *
    Slash(char),  // /

    BitOr,  // |
    Or,     // ||
    BitAnd, // &
    And,    // &&

    LParent(char), // left parenthesis (
    RParent(char), // right parenthesis )
    LBrace(char),  // left brace {
    RBrace(char),  // right brace }
    // left square brackets [
    LSBracket(char),
    // right square brackets ]
    RSBracket(char),

    Lt(String),    // less than <
    LtEQ(String),  // <=
    Gt(String),    // > greater than
    GtEQ(String),  // >=
    EQ(String),    // ==
    NotEQ(String), // !=

    EOF,

    Unkown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Unkown,
    Null,
    Ident,
    Var,
    Assign,
    Or,
    And,
    True,
    False,
    Integer,
    Float,
    LParent,
    RParent,
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::Ident(_) => TokenType::Ident,
            Token::Var => TokenType::Var,
            Token::Assign(_) => TokenType::Assign,
            Token::Or => TokenType::Or,
            Token::And => TokenType::And,
            Token::True => TokenType::True,
            Token::False => TokenType::False,
            Token::Integer(_) => TokenType::Integer,
            Token::Float(_) => TokenType::Float,
            Token::LParent(_) => TokenType::LParent,
            Token::RParent(_) => TokenType::RParent,
            _ => TokenType::Unkown,
        }
    }
}

pub fn get_keyword(kw: &str) -> Option<Token> {
    match kw {
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "var" => Some(Token::Var),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "for" => Some(Token::For),
        "while" => Some(Token::While),
        "return" => Some(Token::Return),
        "print" => Some(Token::Print("print".to_string())),
        _ => None,
    }
}
