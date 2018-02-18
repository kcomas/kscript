#[derive(Debug)]
pub enum RuntimeError {
    CannotLoadCountedMemory,
    CannotLoadFixedMemory,
    CannotIncRefCount,
    CannotDecRefCount,
    TargetIsNotABool,
    CannotLoadAddressIsData,
    CannotReloadData,
    CannotIncData,
}
