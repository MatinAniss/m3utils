use crate::types::MediaType;

#[derive(Debug)]
pub struct Rendition {
    pub media_type: MediaType,
    pub uri: Option<String>,
    /// The group ID that this rendition is associated with.
    pub group_id: String,
    pub language: Option<String>,
    pub associated_language: Option<String>,
    pub name: String,
    pub default: bool,
    pub autoselect: bool,
    pub forced: bool,
    pub instream_id: Option<String>,
    pub characteristics: Option<String>,
    pub channels: Option<String>,
}
