pub type AstBody = Vec<Vec<Ast>>;

#[derive(Debug, Clone)]
pub enum Ast {
    Comment(String),
    Integer(i64),
    Float(f64),
    String(String),
    VarArg {
        name: String,
        index: usize,
    },
    VarLocal {
        name: String,
        index: usize,
    },
    Group(AstBody),
    Function {
        num_locals: usize,
        arguments: AstBody,
        body: AstBody,
    },
    FunctionCall {
        target_local: usize,
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
