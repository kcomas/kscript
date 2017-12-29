use std::collections::HashMap;
use super::error::Error;

#[derive(Debug)]
pub struct SymbolTable {
    // name, index
    functions: HashMap<String, usize>,
    vars: HashMap<String, usize>,
    var_counter: usize,
}

impl<'a> SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            functions: HashMap::new(),
            vars: HashMap::new(),
            var_counter: 0,
        }
    }

    pub fn register_function(&mut self, name: &str, position: usize) -> Result<(), Error<'a>> {
        if let Some(_) = self.functions.get(name) {
            return Err(Error::FunctionDeclared(
                name.to_string(),
                "Function allready declared",
            ));
        }
        self.functions.insert(name.to_string(), position);
        Ok(())
    }
}
