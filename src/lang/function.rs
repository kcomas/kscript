#[derive(Debug)]
pub struct FunctionPointer {
    entry_index: usize,
    number_arguments: usize,
    number_locals: usize,
}
