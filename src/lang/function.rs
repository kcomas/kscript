use super::command::Command;

#[derive(Debug)]
pub struct FunctionLookup {
    functions: Vec<Vec<Command>>,
    current_function: usize,
}

impl FunctionLookup {
    pub fn new() -> FunctionLookup {
        FunctionLookup {
            functions: vec![Vec::new()],
            current_function: 0,
        }
    }

    pub fn get(&self, f_index: usize, c_index: usize) -> Option<&Command> {
        if let Some(fns) = self.functions.get(f_index) {
            if let Some(cmd) = fns.get(c_index) {
                return Some(cmd);
            }
        }
        None
    }

    pub fn add(&mut self) -> usize {
        self.functions.push(Vec::new());
        self.current_function += 1;
        self.current_function
    }

    pub fn update(&mut self, mut commands: Vec<Command>, index: usize) {
        self.functions[index].append(&mut commands);
    }

    pub fn push(&mut self, command: Command, index: usize) {
        self.functions[index].push(command);
    }

    pub fn clear(&mut self, index: usize) {
        self.functions[index] = Vec::new();
    }
}
