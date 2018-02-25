use super::token::{Token, TokenBody};
use super::ast::{Ast, AstBody};
use super::symbol::SymbolTable;
use super::error::JoinerError;

type TokenConvertFn = fn(&Token, &mut SymbolTable, &Vec<Joiner>) -> Result<Ast, JoinerError>;
type TokenVecConverterFn =
    fn(Vec<&Token>, &mut SymbolTable, &Vec<Joiner>) -> Result<Ast, JoinerError>;

pub struct Joiner {
    base: Token,
    base_ast: TokenConvertFn,
    tokens: Vec<Vec<Token>>,
    asts: Vec<Vec<TokenVecConverterFn>>,
}

impl Joiner {
    pub fn new(
        base: Token,
        base_ast: TokenConvertFn,
        tokens: Vec<Vec<Token>>,
        asts: Vec<Vec<TokenVecConverterFn>>,
    ) -> Joiner {
        Joiner {
            base: base,
            base_ast: base_ast,
            tokens: tokens,
            asts: asts,
        }
    }

    pub fn is_match(&self, token: &Token) -> bool {
        self.base.match_type(token)
    }

    pub fn do_match(
        &self,
        current_index: &mut usize,
        tokens: &Vec<Token>,
        symbols: &mut SymbolTable,
        joiners: &Vec<Joiner>,
    ) -> Result<Ast, JoinerError> {
        let mut ast = (self.base_ast)(&tokens[*current_index], symbols, joiners)?;
        *current_index += 1;
        Ok(ast)
    }
}

pub fn create_joiners() -> Vec<Joiner> {
    vec![
        Joiner::new(
            Token::Comment(String::new()),
            |token, _, _| {
                let value = if let Token::Comment(ref comment) = *token {
                    Ast::Comment(comment.clone())
                } else {
                    Ast::Comment(String::new())
                };
                Ok(value)
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::Integer(0),
            |token, _, _| {
                let value = if let Token::Integer(int) = *token {
                    Ast::Integer(int)
                } else {
                    Ast::Integer(0)
                };
                Ok(value)
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::Float(0.0),
            |token, _, _| {
                let value = if let Token::Float(float) = *token {
                    Ast::Float(float)
                } else {
                    Ast::Float(0.0)
                };
                Ok(value)
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::String(String::new()),
            |token, _, _| {
                let value = if let Token::String(ref string) = *token {
                    Ast::String(string.clone())
                } else {
                    Ast::String(String::new())
                };
                Ok(value)
            },
            Vec::new(),
            Vec::new(),
        ),
    ]
}

pub fn join_tokens(
    tokens: &TokenBody,
    symbols: &mut SymbolTable,
    joiners: &Vec<Joiner>,
) -> Result<AstBody, JoinerError> {
    let mut current_ast: Vec<Ast> = Vec::new();
    let mut ast_body = Vec::new();

    for token_section in tokens {
        let mut current_token_counter = 0;
        while current_token_counter < token_section.len() {
            let mut joiner_index = 0;
            while joiner_index < joiners.len() {
                if joiners[joiner_index].is_match(&token_section[current_token_counter]) {
                    current_ast.push(joiners[joiner_index].do_match(
                        &mut current_token_counter,
                        token_section,
                        symbols,
                        joiners,
                    )?);
                    break;
                } else {
                    joiner_index += 1;
                }
            }

            if joiner_index == joiners.len() {
                ast_body.push(current_ast);
                println!("{:?}", ast_body);
                return Err(JoinerError::InvalidToken);
            }
        }
        ast_body.push(current_ast);
        current_ast = Vec::new();
    }

    Ok(ast_body)
}
