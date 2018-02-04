use std::collections::HashMap;
use super::ast::Ast;

type SymbolMap = HashMap<String, usize>;

pub trait STable {
    fn getsert(&mut self, name: &str) -> Ast;
}

#[derive(Debug)]
pub struct ArgSymbolTable<'a> {
    args: &'a mut SymbolMap,
    arg_index_counter: &'a mut usize,
}

impl<'a> ArgSymbolTable<'a> {
    pub fn new(args: &'a mut SymbolMap, arg_index_counter: &'a mut usize) -> ArgSymbolTable<'a> {
        ArgSymbolTable {
            args: args,
            arg_index_counter: arg_index_counter,
        }
    }
}

impl<'a> STable for ArgSymbolTable<'a> {
    fn getsert(&mut self, name: &str) -> Ast {
        if let Some(idx) = self.args.get(name) {
            return Ast::VarArg(*idx);
        }
        let current_index = *self.arg_index_counter;
        self.args.insert(name.to_string(), current_index);
        *self.arg_index_counter += 1;
        Ast::VarArg(current_index)
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    locals: SymbolMap,
    args: SymbolMap,
    local_index_counter: usize,
    arg_index_counter: usize,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            locals: HashMap::new(),
            args: HashMap::new(),
            local_index_counter: 0,
            arg_index_counter: 0,
        }
    }

    pub fn get_arg_table(&mut self) -> ArgSymbolTable {
        ArgSymbolTable::new(&mut self.args, &mut self.arg_index_counter)
    }
}

impl STable for SymbolTable {
    fn getsert(&mut self, name: &str) -> Ast {
        if let Some(idx) = self.locals.get(name) {
            return Ast::VarLocal(*idx);
        } else if let Some(idx) = self.args.get(name) {
            return Ast::VarArg(*idx);
        }
        let current_index = self.local_index_counter;
        self.locals.insert(name.to_string(), current_index);
        self.local_index_counter += 1;
        Ast::VarLocal(current_index)
    }
}
