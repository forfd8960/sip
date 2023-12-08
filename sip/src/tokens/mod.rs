#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    Ident(String),
    Integer(i64),
    Float(f64),
    SString(String),
    True(bool),
    False(bool),

    Var(String),   // keyword: var
    Print(String), // keyword: print

    Assign(String), // =
    Plus(String),   // +
    Minus(String),  // -
    Star(String),   // *
    Slash(String),  // /

    Lt(String),    // less than <
    LtEQ(String),  // <=
    Gt(String),    // > greater than
    GtEQ(String),  // >=
    EQ(String),    // ==
    NotEQ(String), // !=

    EOF,
}
