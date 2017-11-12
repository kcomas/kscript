

#[derive(Debug, Clone, PartialEq)]
pub enum SystemCommand {
    Exit(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    End,
    Const(String),
    Var(String),
    Integer(i64),
    Float(f64),
    Assign,
    Addition,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponent,
    Math(Vec<Token>),
    IoWrite,
    IoAppend,
    IoRead,
    IoReadAppend,
    Comment(String),
    File(String),
    String(String),
    Array(Vec<Token>),
    Dict(Vec<Token>, Vec<Token>),
    ObjectAccess(Box<Token>, Box<Token>),
    Bool(bool),
    Equals,
    EqualOrGreater,
    EqualOrLess,
    Greater,
    Less,
    And,
    Or,
    Conditional(Box<Token>, Box<Token>, Box<Token>),
    If(Box<Token>, Vec<Token>, Vec<Token>),
    Loop(Box<Token>, Vec<Token>),
    Ref(Box<Token>),
    Function(Vec<Token>, Vec<Token>),
    FunctionCall(Box<Token>, Vec<Token>),
    System(SystemCommand),
}
