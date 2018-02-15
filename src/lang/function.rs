#[derive(Debug, Clone)]
pub struct FunctionPointer {
    pub current_command_index: usize,
    pub num_arguments: usize,
    pub num_locals: usize,
}
