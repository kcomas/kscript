
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
    SetRegisterValue(DataHolder),
    ClearRegitser,
    Run,
    Assign(DataHolder),
    IoWrite(DataHolder),
}
