#[derive(Debug)]
pub enum RuntimeError {
    CannotPopStackEmpty,
    InvalidCommandIndex,
    CallsEmpty,
}
