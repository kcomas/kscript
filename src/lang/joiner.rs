use super::ast::{Ast, AstBody};
use super::error::JoinError;

pub fn join_tokens(ast: &AstBody) -> Result<AstBody, JoinError> {
    let mut joined_tokens = Vec::new();
    let mut current_joined_tokens = Vec::new();

    for x in 0..ast.len() {
        if ast[x].len() == 0 {
            continue;
        }
        let mut y = 0;
        while y < ast[x].len() {
            let current = &ast[x][y];
            let token = match *current {
                Ast::Group(ref group) => load_function(group, ast[x].get(y + 1), &mut y)?,
                Ast::Var(ref var_name) => load_function_call(var_name, ast[x].get(y + 1), &mut y)?,
                Ast::If => load_if_statement(ast[x].get(y + 1), &mut y)?,
                Ast::Call => load_self_function_call(ast[x].get(y + 1), &mut y)?,
                _ => current.clone(),
            };
            current_joined_tokens.push(token);
            y += 1;
        }
        joined_tokens.push(current_joined_tokens.clone());
        current_joined_tokens.clear();
    }

    Ok(joined_tokens)
}

fn load_function(group: &AstBody, next: Option<&Ast>, y: &mut usize) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Ast::Block(ref block) = *next_token {
            *y += 1;
            return Ok(Ast::Function(join_tokens(group)?, join_tokens(block)?));
        }
    }
    Ok(Ast::Group(join_tokens(group)?))
}

fn load_function_call(
    var_name: &String,
    next: Option<&Ast>,
    y: &mut usize,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Ast::Group(ref group) = *next_token {
            *y += 1;
            return Ok(Ast::FunctionCall(var_name.clone(), join_tokens(group)?));
        }
    }
    Ok(Ast::Var(var_name.clone()))
}

fn load_self_function_call(next: Option<&Ast>, y: &mut usize) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Ast::Group(ref group) = *next_token {
            *y += 1;
            return Ok(Ast::SelfFuctionCall(join_tokens(group)?));
        }
    }
    Err(JoinError::InvalidSelfCallStatement)
}

fn load_if_statement(next: Option<&Ast>, y: &mut usize) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Ast::Block(ref block) = *next_token {
            *y += 1;
            return Ok(Ast::IfStatement(join_tokens(block)?));
        }
    }
    Err(JoinError::InvalidIfStatement)
}
