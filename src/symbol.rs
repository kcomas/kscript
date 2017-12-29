use std::collections::HashMap;
use super::error::Error;

#[derive(Debug)]
pub struct SymbolTable {
    // name, index
    functions: HashMap<String, usize>,
    vars: HashMap<String, usize>,
    var_counter: usize,
    root: bool,
}

impl<'a> SymbolTable {
    pub fn new(root: bool) -> SymbolTable {
        SymbolTable {
            functions: HashMap::new(),
            vars: HashMap::new(),
            var_counter: 0,
            root: root,
        }
    }

    pub fn register_function(&mut self, name: &str, position: usize) -> Result<bool, Error<'a>> {
        if let Some(_) = self.functions.get(name) {
            return Err(Error::FunctionDeclared(
                name.to_string(),
                "Function allready declared",
            ));
        }
        self.functions.insert(name.to_string(), position);
        if name == "main" {
            if !self.root {
                return Err(Error::CannotDeclareSubMain("Main can only be top level"));
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn register_var(&mut self, name: &str) -> Result<(), Error<'a>> {
        if let Some(_) = self.vars.get(name) {
            return Err(Error::VarDeclared(
                name.to_string(),
                "Var allready declared",
            ));
        }
        self.vars.insert(name.to_string(), self.var_counter);
        self.var_counter += 1;
        Ok(())
    }
}
