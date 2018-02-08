#[derive(Debug)]
pub enum RuntimeError {
    InvalidMemoryAccess,
    InvalidRefInc,
    InvalidRefDec,
}
