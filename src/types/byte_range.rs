#[derive(Debug)]
pub struct ByteRange {
    pub length: u64,
    pub offset: Option<u64>,
}
