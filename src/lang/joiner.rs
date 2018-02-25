use super::token::{Token, TokenBody};
use super::ast::{Ast, AstBody};
use super::symbol::SymbolTable;
use super::error::JoinerError;

type TokenConvertFn = fn(&Token, &mut SymbolTable, &Vec<Joiner>) -> Result<Ast, JoinerError>;
type TokenVecConverterFn =
    fn(&Vec<Ast>, &Vec<Token>, &mut SymbolTable, &Vec<Joiner>) -> Result<Ast, JoinerError>;

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
        if self.tokens.len() == 0 {
            return Ok(ast);
        }

        let mut prev_ast = vec![ast];
        let mut outer_index = 0;

        while outer_index < self.tokens.len() {
            let mut inner_index = 0;
            while inner_index < self.tokens[outer_index].len() {
                if self.tokens[outer_index][inner_index].match_type(&tokens[*current_index]) {
                    let next_ast_match =
                        (self.asts[outer_index][inner_index])(&prev_ast, tokens, symbols, joiners)?;
                    prev_ast.push(next_ast_match);
                    *current_index += 1;
                    break;
                }
                inner_index += 1;
            }
            if inner_index == self.tokens[outer_index].len() {
                break;
            }
            outer_index += 1;
        }

        if let Some(item) = prev_ast.pop() {
            Ok(item)
        } else {
            Err(JoinerError::AstMultiMatchVecEmpty)
        }
    }
}

pub fn create_joiners() -> Vec<Joiner> {
    vec![
        Joiner::new(
            Token::Comment(String::new()),
            |token, _, _| {
                if let Token::Comment(ref comment) = *token {
                    Ok(Ast::Comment(comment.clone()))
                } else {
                    Err(JoinerError::TokenFnMismatch)
                }
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::Integer(0),
            |token, _, _| {
                if let Token::Integer(int) = *token {
                    Ok(Ast::Integer(int))
                } else {
                    Err(JoinerError::TokenFnMismatch)
                }
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::Float(0.0),
            |token, _, _| {
                if let Token::Float(float) = *token {
                    Ok(Ast::Float(float))
                } else {
                    Err(JoinerError::TokenFnMismatch)
                }
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::String(String::new()),
            |token, _, _| {
                if let Token::String(ref string) = *token {
                    Ok(Ast::String(string.clone()))
                } else {
                    Err(JoinerError::TokenFnMismatch)
                }
            },
            Vec::new(),
            Vec::new(),
        ),
        Joiner::new(
            Token::Var(String::new()),
            |token, symbols, _| {
                if let Token::Var(ref name) = *token {
                    Ok(Ast::Var(symbols.getsert(name)))
                } else {
                    Err(JoinerError::TokenFnMismatch)
                }
            },
            vec![vec![Token::Group(Vec::new())]],
            vec![vec![|prev_ast, tokens, symbols, joiners| Ok(Ast::Greater)]],
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
                current_token_counter += 1;
                // ast_body.push(current_ast);
                // println!("{:?}", ast_body);
                // return Err(JoinerError::InvalidToken);
            }
        }
        ast_body.push(current_ast);
        current_ast = Vec::new();
    }

    Ok(ast_body)
}
