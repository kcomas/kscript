#[derive(Debug, Clone)]
pub struct FunctionPointer {
    pub return_address: usize,
    pub command_index: usize,
    pub enter_stack_len: usize,
    pub num_arguments: usize,
    pub num_locals: usize,
    pub function_length: usize,
}
