use crate::types::SessionDataEntry;

#[derive(Debug)]
pub struct SessionData {
    pub data_id: String,
    pub entry: SessionDataEntry,
    pub language: Option<String>,
}
