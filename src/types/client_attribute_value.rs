#[derive(Debug)]
pub enum ClientAttributeValue {
    String(String),
    Hexadecimal(String),
    F64(f64),
}
