use std::collections::HashMap;
use super::ast::Var;

pub type SymbolMap = HashMap<String, usize>;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    locals: SymbolMap,
    local_index_counter: usize,
    args: SymbolMap,
    args_index_counter: usize,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            locals: HashMap::new(),
            local_index_counter: 0,
            args: HashMap::new(),
            args_index_counter: 0,
        }
    }

    pub fn get_total_locals(&self) -> usize {
        self.local_index_counter
    }

    pub fn add_arg(&mut self, name: &str) -> usize {
        let index = self.args_index_counter;
        self.args.insert(name.to_string(), index);
        self.args_index_counter += 1;
        index
    }

    pub fn getsert(&mut self, name: &str) -> Var {
        if let Some(index) = self.args.get(name) {
            return Var::Arg(*index);
        } else if let Some(index) = self.locals.get(name) {
            return Var::Local(*index);
        }
        let index = self.local_index_counter;
        self.locals.insert(name.to_string(), index);
        self.local_index_counter += 1;
        Var::Local(index)
    }
}
