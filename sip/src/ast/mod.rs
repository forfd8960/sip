pub trait Node {
    fn token_literal() -> String;
}

pub struct Program {}

impl Node for Program {
    fn token_literal() -> String {
        "".to_string()
    }
}
