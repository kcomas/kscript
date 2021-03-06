use super::ast::{Ast, AstArgs, AstBody};
use super::super::super::error::ParserError;
use super::super::symbol::{SymbolTable, SymbolType};

pub fn shunt_yard(ast: &mut Vec<Ast>, symbols: &mut SymbolTable) -> Result<Vec<Ast>, ParserError> {
    ast.reverse();
    let mut op_stack: Vec<Ast> = Vec::new();
    let mut result_stack = Vec::new();
    while let Some(mut op) = ast.pop() {
        let presedence = op.presedence();
        if presedence == 0 {
            continue;
        }
        // shunt body
        op = match op {
            Ast::Function(ref mut args, ref mut body) => {
                let mut new_symbols = SymbolTable::new();
                new_symbols.set_arg_mode();
                let new_args = shunt_args(args, &mut new_symbols)?;
                new_symbols.set_local_mode();
                let new_body = shunt_body(body, &mut new_symbols)?;
                Ast::Function(new_args, new_body)
            }
            Ast::Group(ref mut body) => Ast::Group(shunt_body(body, symbols)?),
            Ast::Array(ref mut items) => Ast::Array(shunt_args(items, symbols)?),
            Ast::Access(ref mut body) => Ast::Access(shunt_body(body, symbols)?),
            Ast::AccessAssign(ref mut access_body, ref mut assign_body) => Ast::AccessAssign(
                shunt_body(access_body, symbols)?,
                shunt_body(assign_body, symbols)?,
            ),
            Ast::AccessCall(ref mut access_body, ref mut items) => Ast::AccessCall(
                shunt_body(access_body, symbols)?,
                shunt_args(items, symbols)?,
            ),
            Ast::FunctionCall(ref mut args) => Ast::FunctionCall(shunt_args(args, symbols)?),
            Ast::FunctionSelfCall(ref mut args) => {
                Ast::FunctionSelfCall(shunt_args(args, symbols)?)
            }
            Ast::If(ref mut body) => Ast::If(shunt_body(body, symbols)?),
            Ast::Assign(ref mut body) => Ast::Assign(shunt_body(body, symbols)?),
            _ => op,
        };
        if presedence == 1 {
            let do_push = match op.has_var_name() {
                Some(name) => {
                    match symbols.get(name) {
                        SymbolType::Arg(id) => result_stack.push(Ast::VarArg(name.to_string(), id)),
                        SymbolType::Local(id) => {
                            result_stack.push(Ast::VarLocal(name.to_string(), id))
                        }
                    };
                    false
                }
                None => true,
            };
            if do_push {
                result_stack.push(op);
            }
        } else {
            let mut clear_op_stack = false;
            let mut add_last = false;
            if let Some(last) = op_stack.last() {
                // compare presedence
                let last_presedence = last.presedence();
                if presedence < last_presedence {
                    clear_op_stack = true;
                } else if presedence == last_presedence {
                    add_last = true;
                }
            }
            if clear_op_stack {
                op_stack.reverse();
                result_stack.append(&mut op_stack);
            } else if add_last {
                result_stack.push(op_stack.pop().unwrap());
            }
            op_stack.push(op);
        }
    }
    if op_stack.len() > 0 {
        op_stack.reverse();
        result_stack.append(&mut op_stack);
    }
    Ok(result_stack)
}

fn shunt_args(args: &mut AstArgs, symbols: &mut SymbolTable) -> Result<AstArgs, ParserError> {
    let mut new_args = Vec::new();
    for arg in args.iter_mut() {
        let mut new_statements = Vec::new();
        for statements in arg.iter_mut() {
            new_statements.push(shunt_yard(statements, symbols)?);
        }
        new_args.push(new_statements);
    }
    Ok(new_args)
}

fn shunt_body(body: &mut AstBody, symbols: &mut SymbolTable) -> Result<AstBody, ParserError> {
    let mut new_body = Vec::new();
    for statement in body.iter_mut() {
        new_body.push(shunt_yard(statement, symbols)?);
    }
    Ok(new_body)
}
