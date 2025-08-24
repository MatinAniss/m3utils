use crate::types::{ByteRange, DateRange, DateTime, Key, Map};

#[derive(Debug)]
pub struct Segment {
    pub uri: String,
    pub duration: f64,
    pub title: Option<String>,
    pub byte_range: Option<ByteRange>,
    pub discontinuity: bool,
    pub key: Option<Key>,
    pub map: Option<Map>,
    pub program_date_time: Option<DateTime>,
    pub date_range: Option<DateRange>,
}
