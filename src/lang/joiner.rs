use super::token::{Token, TokenBody};
use super::ast::{Ast, AstBody};
use super::error::JoinError;

pub fn join_tokens(tokens: &TokenBody) -> Result<AstBody, JoinError> {
    let mut joined_ast = Vec::new();
    let mut current_joined_ast = Vec::new();

    for x in 0..tokens.len() {
        if tokens[x].len() == 0 {
            continue;
        }
        let mut y = 0;
        while y < tokens[x].len() {
            let current = &tokens[x][y];
            let ast = match *current {
                Token::Comment(ref comment) => Ast::Comment(comment.clone()),
                Token::Integer(int) => Ast::Integer(int),
                Token::Float(float) => Ast::Float(float),
                Token::Var(ref var_name) => {
                    load_function_call(var_name, tokens[x].get(y + 1), &mut y)?
                }
                Token::Group(ref group) => load_function(group, tokens[x].get(y + 1), &mut y)?,
                Token::Block(_) => return Err(JoinError::BlockShouldNotBeReached),
                Token::If => load_if_statement(tokens[x].get(y + 1), &mut y)?,
                Token::Call => load_self_function_call(tokens[x].get(y + 1), &mut y)?,
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

fn load_function(group: &TokenBody, next: Option<&Token>, y: &mut usize) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Block(ref block) = *next_token {
            *y += 1;
            return Ok(Ast::Function(join_tokens(group)?, join_tokens(block)?));
        }
    }
    Ok(Ast::Group(join_tokens(group)?))
}

fn load_function_call(
    var_name: &String,
    next: Option<&Token>,
    y: &mut usize,
) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Group(ref group) = *next_token {
            *y += 1;
            return Ok(Ast::FunctionCall(var_name.clone(), join_tokens(group)?));
        }
    }
    Ok(Ast::Var(var_name.clone()))
}

fn load_self_function_call(next: Option<&Token>, y: &mut usize) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Group(ref group) = *next_token {
            *y += 1;
            return Ok(Ast::SelfFuctionCall(join_tokens(group)?));
        }
    }
    Err(JoinError::InvalidSelfCallStatement)
}

fn load_if_statement(next: Option<&Token>, y: &mut usize) -> Result<Ast, JoinError> {
    if let Some(next_token) = next {
        if let Token::Block(ref block) = *next_token {
            *y += 1;
            return Ok(Ast::IfStatement(join_tokens(block)?));
        }
    }
    Err(JoinError::InvalidIfStatement)
}
