
#[derive(Debug, Clone)]
pub enum DataType {
    Integer(i64),
    Float(f64),
    String(String),
    File(String),
    Array(Vec<DataType>),
}

#[derive(Debug, Clone)]
pub enum DataHolder {
    Var(String),
    Const(String),
    Annon(DataType),
}

#[derive(Debug, Clone)]
pub enum Command {
    AddRegister,
    SetRegisterValue(usize, DataHolder),
    // target source
    Run(usize, usize),
    Assign(DataHolder, usize),
    IoWrite(DataHolder, usize),
}
