#[derive(Debug, Clone)]
pub struct FunctionPointer {
    pub entry_command_index: usize,
    pub current_command_index: usize,
    pub num_arguments: usize,
    pub num_locals: usize,
    pub function_length: usize,
}
