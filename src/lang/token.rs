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

macro_rules! match_token_type {
    ($self: expr, $other: expr, $($token:pat),*) => {
        match *$self {
            $(
                $token => if let $token = *$other { true } else { false },
            )*
        }
    }
}

impl Token {
    pub fn match_type(&self, other: &Token) -> bool {
        match_token_type!(
            self,
            other,
            Token::Comment(_),
            Token::Integer(_),
            Token::Float(_),
            Token::String(_),
            Token::Var(_),
            Token::Group(_),
            Token::Block(_),
            Token::If,
            Token::Add,
            Token::Sub,
            Token::CallSelf,
            Token::Return,
            Token::Assign,
            Token::Equals,
            Token::EqualsGreater,
            Token::EqualsLess,
            Token::Greater,
            Token::Less,
            Token::Not,
            Token::NotEquals,
            Token::IoWrite,
            Token::IoAppend
        )
    }
}
