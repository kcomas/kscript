
#[derive(Debug, Clone)]
pub enum DataType {
    Integer(i64),
    Float(f64),
    String(String),
    File(String),
}

#[derive(Debug, Clone)]
pub enum DataHolder {
    Var(String),
    Const(String),
    Anon(DataType),
    Array(Vec<DataHolder>),
}

#[derive(Debug, Clone)]
pub enum Command {
    SetRegister(usize, DataHolder),
    ClearRegisters,
    // target source
    Run(usize, usize),
    Assign(usize, usize),
    IoWrite(usize, usize),
}
