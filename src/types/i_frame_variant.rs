use crate::types::{HDCPLevel, Resolution};

#[derive(Debug)]
pub struct IFrameVariant {
    pub bandwidth: u64,
    pub average_bandwidth: Option<u64>,
    pub codecs: Option<String>,
    pub resolution: Option<Resolution>,
    pub hdcp_level: Option<HDCPLevel>,
    /// The group ID associating this variant with a video rendition group.
    pub video: Option<String>,
    pub uri: String,
}
