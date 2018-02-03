pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    Var(usize),
    Group(AstBody), // ()
    Function(AstBody, AstBody),
    FunctionCall(String, AstBody),
    SelfFuctionCall(AstBody),
    IfStatement(AstBody),
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
