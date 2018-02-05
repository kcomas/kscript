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
                    load_var(var_name, tokens[x].get(y + 1), &mut y, symbol_table)?
                }
                Token::Group(ref group) => load_function(
                    group,
                    tokens[x].get(y + 1),
                    tokens[x].get(y + 2),
                    &mut y,
                    symbol_table,
                )?,
                Token::Block(_) => return Err(JoinError::BlockShouldNotBeReached),
                Token::If => load_if_statement(tokens[x].get(y + 1), &mut y, symbol_table)?,
                Token::Call => load_self_function_call(tokens[x].get(y + 1), &mut y, symbol_table)?,
                Token::Add => Ast::Add,
                Token::Sub => Ast::Sub,
                Token::Return => Ast::Return,
                Token::Assign => return Err(JoinError::AssignShouldNotBeReached),
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
    peek: Option<&Token>,
    y: &mut usize,
    symbol_table: &mut T,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Block(ref block) = *next_token {
            *y += 1;
            let mut sub_table = SymbolTable::new();
            let arg_index = {
                let mut arg_table = sub_table.get_arg_table();
                let mut arg_index = 0;
                for arg in group.iter() {
                    if arg.len() != 1 {
                        return Err(JoinError::InvalidFunctionArgument);
                    }
                    let name = match arg.get(0) {
                        Some(ast) => match *ast {
                            Token::Var(ref name) => name,
                            _ => return Err(JoinError::InvalidFunctionArgument),
                        },
                        _ => return Err(JoinError::InvalidFunctionArgument),
                    };
                    arg_index = match arg_table.getsert(name) {
                        Ast::VarArg(index) => index,
                        _ => return Err(JoinError::InvalidFunctionArgument),
                    };
                }
                arg_index
            };
            if let Some(peek_token) = peek {
                if let Token::Group(ref group) = *peek_token {
                    *y += 1;
                    return Ok(Ast::ImmidiateFunction(
                        arg_index + 1,
                        join_tokens(block, &mut sub_table)?,
                        join_tokens(group, symbol_table)?,
                    ));
                }
            }
            return Ok(Ast::Function(
                arg_index + 1,
                join_tokens(block, &mut sub_table)?,
            ));
        }
    }
    Ok(Ast::Group(join_tokens(group, symbol_table)?))
}

fn load_var<T: STable>(
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
        } else if let Token::Assign = *next_token {
            *y += 1;
            match symbol_table.getsert(var_name) {
                Ast::VarLocal(index) => return Ok(Ast::SaveLocal(index)),
                Ast::VarArg(index) => return Ok(Ast::SaveArg(index)),
                _ => return Err(JoinError::InvalidAssignVarSymbol),
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
