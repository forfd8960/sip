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

    Var(String),   // keyword: var
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
}

pub fn get_keyword(kw: &str) -> Option<Token> {
    match kw {
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "var" => Some(Token::Var("var".to_string())),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "for" => Some(Token::For),
        "while" => Some(Token::While),
        "return" => Some(Token::Return),
        "print" => Some(Token::Var("print".to_string())),
        _ => None,
    }
}
