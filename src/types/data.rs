use crate::types::DataEntry;

#[derive(Debug)]
pub struct Data {
    pub data_id: String,
    pub entry: DataEntry,
    pub language: Option<String>,
}
