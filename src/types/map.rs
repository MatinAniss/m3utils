use crate::types::ByteRange;

#[derive(Debug)]
pub struct Map {
    pub uri: String,
    pub byterange: Option<ByteRange>,
}
