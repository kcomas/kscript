use super::ast::Ast;
use super::super::super::error::ParserError;

pub fn shunt_yard(ast: &mut Vec<Ast>) -> Result<Vec<Ast>, ParserError> {
    ast.reverse();
    let mut op_stack: Vec<Ast> = Vec::new();
    let mut result_stack = Vec::new();
    while let Some(op) = ast.pop() {
        let presedence = op.presedence();
        if presedence == 0 {
            continue;
        }
        if presedence == 1 {
            result_stack.push(op);
        } else {
            if op.has_body() {
                // shunt body
            }
            let mut do_push = true;
            if let Some(last) = op_stack.last() {
                // compare presedence
                let last_presedence = last.presedence();
                if last_presedence > presedence {
                    do_push = false;
                }
            }
            if do_push {
                op_stack.push(op);
            } else {
                let last = op_stack.pop().unwrap();
                result_stack.push(last);
                op_stack.push(op);
            }
        }
    }
    if op_stack.len() > 0 {
        result_stack.append(&mut op_stack);
    }
    Ok(result_stack)
}
