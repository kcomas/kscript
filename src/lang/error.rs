#[derive(Debug)]
pub enum RuntimeError {
    CannotGetCountedMemoryItem,
    CannotGetFixedMemoryItem,
    CannotIncRef,
    CannotDecRef,
}
