pub type TokenBody = Vec<Vec<Token>>;

#[derive(Debug, Clone)]
pub enum Token {
    Comment(String),
    Integer(i64),
    Float(f64),
    Var(String),
    Group(TokenBody), // ()
    Block(TokenBody), // {}
    If,
    Add,
    Sub,
    Call,
    Return,
    Assign,
    Equals,
    EqualsGreater,
    EqualsLess,
    Greater,
    Less,
    IoWrite,
    IoAppend,
}
