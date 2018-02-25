use super::symbol::SymbolTable;

pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug)]
pub enum Var {
    Arg(usize),
    Local(usize),
}

#[derive(Debug)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    String(String),
    Var(Var),
    Group(AstBody),
    Function {
        num_locals: usize,
        num_arguments: usize,
        body: AstBody,
        symbols: SymbolTable,
    },
    FunctionCall {
        target: Box<Ast>,
        arguments: AstBody,
    },
    SelfFunctionCall(AstBody),
    If {
        conditional: AstBody,
        body: AstBody,
    },
    Equals,
    EqualsGreater,
    EqualsLess,
    Less,
    Greater,
    Not,
    NotEquals,
    IoWrite,
    IoAppend,
}
