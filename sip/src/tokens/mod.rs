#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Integer(i64),
    Float(f64),
    SString(String),

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
}
