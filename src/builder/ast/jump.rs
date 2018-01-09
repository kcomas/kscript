use std::collections::HashMap;
use super::ast::{Ast, AstArgs, AstBody};
use super::super::super::error::ParserError;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Arg(usize),
    Local(usize),
}

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, SymbolType>,
    counter: usize,
    mode: SymbolType,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
            counter: 0,
            mode: SymbolType::Local(0),
        }
    }

    pub fn set_arg_mode(&mut self) {
        self.mode = SymbolType::Arg(0);
    }

    pub fn set_local_mode(&mut self) {
        self.mode = SymbolType::Local(0);
    }

    pub fn get(&mut self, name: &str) -> SymbolType {
        if let Some(symbol_type) = self.table.get(name) {
            return symbol_type.clone();
        }
        let symbol_type = match self.mode {
            SymbolType::Arg(_) => SymbolType::Arg(self.counter),
            SymbolType::Local(_) => SymbolType::Local(self.counter),
        };
        self.table.insert(name.to_string(), symbol_type.clone());
        self.counter += 1;
        symbol_type
    }
}

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
            Ast::FunctionCall(ref mut args) => Ast::FunctionCall(shunt_args(args, symbols)?),
            Ast::If(ref mut body) => Ast::If(shunt_body(body, symbols)?),
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
                while let Some(last) = op_stack.pop() {
                    result_stack.push(last);
                }
                op_stack.push(op);
            }
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
