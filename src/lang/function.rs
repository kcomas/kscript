#[derive(Debug)]
pub struct FunctionPointer {
    pub entry_index: usize,
    pub number_arguments: usize,
    pub number_locals: usize,
}
