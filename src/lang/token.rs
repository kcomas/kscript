pub type TokenBody = Vec<Vec<Token>>;

#[derive(Debug, Clone)]
pub enum Token {
    Comment(String),
    Integer(i64),
    Float(f64),
    String(String),
    Var(String),
    Group(TokenBody), // ()
    Block(TokenBody), // {}
    If,
    Add,
    Sub,
    CallSelf,
    Return,
    Assign,
    Equals,
    EqualsGreater,
    EqualsLess,
    Greater,
    Less,
    Not,
    NotEquals,
    IoWrite,
    IoAppend,
}
