

#[derive(Debug, Clone)]
pub enum DataType {
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<DataType>),
}

#[derive(Debug, Clone)]
pub enum DataHolder {
    Var(String),
    Const(String),
    Annon(DataType),
}

#[derive(Debug, Clone)]
pub enum Action {
    Declare(DataHolder),
    Run(DataHolder),
}

#[derive(Debug, Clone)]
pub enum Command {
    DoAction(Action),
    Assign(Action, Action),
}
