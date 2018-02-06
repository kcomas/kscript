#[derive(Debug)]
pub enum RuntimeError {
    NoMoreFrames,
    NoMoreCommands,
    StackEmpty,
}
