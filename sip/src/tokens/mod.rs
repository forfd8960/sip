#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    WhiteSpace,
    Ident(String),
    Integer(i64),
    Float(f64),
    SString(String),
    True(bool),
    False(bool),

    Var(String),   // keyword: var
    Print(String), // keyword: print

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
