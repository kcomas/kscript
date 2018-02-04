pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    VarLocal(usize),
    VarArg(usize),
    Group(AstBody), // ()
    Function(AstBody, AstBody),
    LocalFunctionCall(usize, AstBody),
    ArgFunctionCall(usize, AstBody),
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
