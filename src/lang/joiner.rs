use super::token::{Token, TokenBody};
use super::ast::{Ast, AstBody};
use super::symbol::{STable, SymbolTable};
use super::error::JoinError;

pub fn join_tokens<T: STable>(
    tokens: &TokenBody,
    symbol_table: &mut T,
) -> Result<AstBody, JoinError> {
    let mut joined_ast = Vec::new();
    let mut current_joined_ast = Vec::new();

    for x in 0..tokens.len() {
        if tokens[x].len() == 0 {
            continue;
        }
        let mut y = 0;
        while y < tokens[x].len() {
            let ast = match tokens[x][y] {
                Token::Comment(ref comment) => Ast::Comment(comment.clone()),
                Token::Integer(int) => Ast::Integer(int),
                Token::Float(float) => Ast::Float(float),
                Token::Var(ref var_name) => {
                    load_function_call(var_name, tokens[x].get(y + 1), &mut y, symbol_table)?
                }
                Token::Group(ref group) => {
                    load_function(group, tokens[x].get(y + 1), &mut y, symbol_table)?
                }
                Token::Block(_) => return Err(JoinError::BlockShouldNotBeReached),
                Token::If => load_if_statement(tokens[x].get(y + 1), &mut y, symbol_table)?,
                Token::Call => load_self_function_call(tokens[x].get(y + 1), &mut y, symbol_table)?,
                Token::Add => Ast::Add,
                Token::Sub => Ast::Sub,
                Token::Call => Ast::Call,
                Token::Return => Ast::Return,
                Token::Assign => Ast::Assign,
                Token::Equals => Ast::Equals,
                Token::EqualsGreater => Ast::EqualsGreater,
                Token::EqualsLess => Ast::EqualsLess,
                Token::Greater => Ast::Greater,
                Token::Less => Ast::Less,
                Token::IoWrite => Ast::IoWrite,
                Token::IoAppend => Ast::IoAppend,
            };
            current_joined_ast.push(ast);
            y += 1;
        }
        joined_ast.push(current_joined_ast.clone());
        current_joined_ast.clear();
    }

    Ok(joined_ast)
}

fn load_function<T: STable>(
    group: &TokenBody,
    next: Option<&Token>,
    y: &mut usize,
    symbol_table: &mut T,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Block(ref block) = *next_token {
            *y += 1;
            let mut sub_table = SymbolTable::new();
            return Ok(Ast::Function(
                join_tokens(group, &mut sub_table.get_arg_table())?,
                join_tokens(block, &mut sub_table)?,
            ));
        }
    }
    Ok(Ast::Group(join_tokens(group, symbol_table)?))
}

fn load_function_call<T: STable>(
    var_name: &str,
    next: Option<&Token>,
    y: &mut usize,
    symbol_table: &mut T,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Group(ref group) = *next_token {
            *y += 1;
            match symbol_table.getsert(var_name) {
                Ast::VarLocal(index) => {
                    return Ok(Ast::LocalFunctionCall(
                        index,
                        join_tokens(group, symbol_table)?,
                    ));
                }
                Ast::VarArg(index) => {
                    return Ok(Ast::ArgFunctionCall(
                        index,
                        join_tokens(group, symbol_table)?,
                    ));
                }
                _ => return Err(JoinError::InvalidFunctionVarSymbol),
            }
        }
    }
    Ok(symbol_table.getsert(var_name))
}

fn load_self_function_call<T: STable>(
    next: Option<&Token>,
    y: &mut usize,
    symbol_table: &mut T,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Group(ref group) = *next_token {
            *y += 1;
            return Ok(Ast::SelfFuctionCall(join_tokens(group, symbol_table)?));
        }
    }
    Err(JoinError::InvalidSelfCallStatement)
}

fn load_if_statement<T: STable>(
    next: Option<&Token>,
    y: &mut usize,
    symbol_table: &mut T,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Block(ref block) = *next_token {
            *y += 1;
            return Ok(Ast::IfStatement(join_tokens(block, symbol_table)?));
        }
    }
    Err(JoinError::InvalidIfStatement)
}
