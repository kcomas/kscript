use std::collections::HashMap;
use super::ast::Ast;

pub type SymbolMap = HashMap<String, usize>;

pub trait SymbolTable {
    fn getsert(&mut self, name: &str) -> Ast;
}

#[derive(Debug)]
pub struct ArgLocalTable {
    locals: SymbolMap,
    local_index_counter: usize,
    args: SymbolMap,
    args_index_counter: usize,
}

impl ArgLocalTable {}
