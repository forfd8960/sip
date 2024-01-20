pub mod token1;

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

    Var,   // keyword: var
    Print, // keyword: print()
    If,
    Else,
    For,
    While,
    Return,
    Def, // def

    Assign(char), // =
    Plus(char),   // +
    Minus(char),  // -
    Star(char),   // *
    Slash(char),  // /
    Bang,         // !

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
    String,
    LParent,
    RParent,
    Lt,    // <
    LtEQ,  // <=
    Gt,    // >
    GtEQ,  // >=
    EQ,    // ==
    NotEQ, // !=
    Slash, // /
    Star,  // *
    Plus,  // +
    Minus, // -
    // !
    Bang,
    If,
    Else,
    For,
    Return,
    While,
    Def,
    LBrace,
    RBrace,
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
            Token::SString(_) => TokenType::String,
            Token::LParent(_) => TokenType::LParent,
            Token::RParent(_) => TokenType::RParent,
            Token::Slash(_) => TokenType::Slash,
            Token::Star(_) => TokenType::Star,
            Token::Plus(_) => TokenType::Plus,
            Token::Minus(_) => TokenType::Minus,
            Token::Bang => TokenType::Bang,
            Token::Lt(_) => TokenType::Lt,
            Token::LtEQ(_) => TokenType::LtEQ,
            Token::Gt(_) => TokenType::Gt,
            Token::GtEQ(_) => TokenType::GtEQ,
            Token::EQ(_) => TokenType::EQ,
            Token::NotEQ(_) => TokenType::NotEQ,
            Token::If => TokenType::If,
            Token::Else => TokenType::Else,
            Token::For => TokenType::For,
            Token::While => TokenType::While,
            Token::Return => TokenType::Return,
            Token::Def => TokenType::Def,
            Token::LBrace(_) => TokenType::LBrace,
            Token::RBrace(_) => TokenType::RBrace,
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
        "print" => Some(Token::Print),
        "def" => Some(Token::Def),
        _ => None,
    }
}
