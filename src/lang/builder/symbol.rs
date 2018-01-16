use std::collections::HashMap;

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

    pub fn set_counter(&mut self, counter: usize) {
        self.counter = counter;
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
