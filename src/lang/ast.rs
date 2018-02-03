pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    Var(String),
    Group(AstBody), // ()
    Block(AstBody), // {}
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
    // Joined Ast
    Function(AstBody, AstBody),
    FunctionCall(String, AstBody),
    SelfFuctionCall(AstBody),
    IfStatement(AstBody),
}
