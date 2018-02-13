#[derive(Debug)]
pub enum DataHolder {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
}

#[derive(Debug)]
pub enum RefHolder<'a> {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(&'a String),
}
