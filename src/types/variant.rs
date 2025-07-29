use crate::types::{HDCPLevel, Resolution};

#[derive(Debug)]
pub struct Variant {
    pub uri: String,
    pub bandwidth: u64,
    pub average_bandwidth: Option<u64>,
    pub codecs: String,
    pub resolution: Option<Resolution>,
    pub frame_rate: Option<f64>,
    pub hdcp_level: Option<HDCPLevel>,
    /// The group ID associating this variant with a audio rendition group.
    pub audio: Option<String>,
    /// The group ID associating this variant with a video rendition group.
    pub video: Option<String>,
    /// The group ID associating this variant with a subtitles rendition group.
    pub subtitles: Option<String>,
    /// The group ID associating this variant with a closed captions rendition group.
    pub closed_captions: Option<String>,
}
