use super::ast::{Ast, AstBody};
use super::error::ShuntError;

pub fn shunt_ast(ast: &mut AstBody) -> Result<AstBody, ShuntError> {
    let mut result_ast = Vec::new();
    ast.reverse();
    while let Some(ref mut ast_section) = ast.pop() {
        let result = shunt(ast_section)?;
        if result.len() > 0 {
            result_ast.push(result);
        }
    }
    Ok(result_ast)
}

fn shunt(ast_section: &mut Vec<Ast>) -> Result<Vec<Ast>, ShuntError> {
    ast_section.reverse();
    let mut op_stack: Vec<Ast> = Vec::new();
    let mut result_stack = Vec::new();
    while let Some(mut op) = ast_section.pop() {
        let presedence = op.presedence();
        if presedence == 0 {
            continue;
        }
        op = match op {
            Ast::Group(ref mut group) => Ast::Group(shunt_ast(group)?),
            Ast::Function(num_args, ref mut body) => Ast::Function(num_args, shunt_ast(body)?),
            Ast::LocalFunctionCall(local_index, ref mut body) => {
                Ast::LocalFunctionCall(local_index, shunt_ast(body)?)
            }
            Ast::ArgFunctionCall(arg_index, ref mut body) => {
                Ast::ArgFunctionCall(arg_index, shunt_ast(body)?)
            }
            Ast::SelfFuctionCall(ref mut body) => Ast::SelfFuctionCall(shunt_ast(body)?),
            Ast::IfStatement(ref mut body) => Ast::IfStatement(shunt_ast(body)?),
            _ => op,
        };
        if presedence == 1 {
            result_stack.push(op);
            continue;
        }
        loop {
            let last_presedence = match op_stack.last() {
                Some(ast_item) => ast_item.presedence(),
                None => break,
            };
            if last_presedence >= presedence {
                if let Some(last_op) = op_stack.pop() {
                    result_stack.push(last_op);
                } else {
                    return Err(ShuntError::FaileToPopOperatorStack);
                }
            } else {
                break;
            }
        }
        op_stack.push(op);
    }
    if op_stack.len() > 0 {
        op_stack.reverse();
        result_stack.append(&mut op_stack);
    }
    Ok(result_stack)
}
