use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::error::Error;

#[derive(Debug)]
pub struct SymbolTable {
    // name, index
    functions: Rc<RefCell<HashMap<String, usize>>>,
    vars: HashMap<String, usize>,
    var_counter: usize,
    root: bool,
}

impl<'a> SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            functions: Rc::new(RefCell::new(HashMap::new())),
            vars: HashMap::new(),
            var_counter: 0,
            root: true,
        }
    }

    pub fn get_sub_table(&self) -> SymbolTable {
        SymbolTable {
            functions: Rc::clone(&self.functions),
            vars: HashMap::new(),
            var_counter: 0,
            root: false,
        }
    }

    pub fn register_function(&mut self, name: &str, position: usize) -> Result<bool, Error<'a>> {
        let mut borrowed_functions = self.functions.borrow_mut();
        if let Some(_) = borrowed_functions.get(name) {
            return Err(Error::FunctionDeclared(
                name.to_string(),
                "Function allready declared",
            ));
        }
        borrowed_functions.insert(name.to_string(), position);
        if name == "main" {
            if !self.root {
                return Err(Error::CannotDeclareSubMain("Main can only be top level"));
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn get_function_index(&mut self, name: &str) -> Result<usize, Error<'a>> {
        if let Some(index) = self.functions.borrow_mut().get(name) {
            return Ok(*index);
        }
        Err(Error::FunctionNotDeclared(
            name.to_string(),
            "Function not declared",
        ))
    }

    pub fn get_main(&self) -> Result<usize, Error<'a>> {
        if let Some(index) = self.functions.borrow_mut().get("main") {
            return Ok(*index);
        }
        Err(Error::MainNotDeclared("Main not declared"))
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

    pub fn get_var_index(&mut self, name: &str) -> Result<usize, Error<'a>> {
        if let Some(index) = self.vars.get(name) {
            return Ok(*index);
        }
        // add the var
        self.register_var(name)?;
        self.get_var_index(name)
    }
}
