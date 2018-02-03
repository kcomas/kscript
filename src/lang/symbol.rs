use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, usize>,
    symbol_index_counter: usize,
}

impl SymbolTable {}
