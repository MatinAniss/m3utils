use std::str::FromStr;

use nom::{
    Parser,
    branch::alt,
    character::complete::{char, line_ending, not_line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::preceded,
};

use crate::{
    tags::{
        TAG_PARSERS, Tags,
        basic::BasicTags,
        master_playlist::{
            MasterPlaylistTags, ext_x_i_frame_stream_inf::ExtXIFrameStreamInfHDCPLevel,
            ext_x_media::ExtXMediaMediaType, ext_x_session_data::ExtXSessionDataEntry,
            ext_x_session_key::ExtXSessionKeyMethod, ext_x_stream_inf::ExtXStreamInfHDCPLevel,
        },
        playlist::PlaylistTags,
    },
    version::Version,
};

pub type GroupId = String;

#[derive(Debug)]
pub struct MasterPlaylist {
    pub version: Version,
    pub variants: Vec<Variant>,
    pub i_frame_variants: Vec<IFrameVariant>,
    pub renditions: Vec<Rendition>,
    pub session_data: Vec<SessionData>,
    pub session_keys: Vec<SessionKey>,
    pub independent_segments: bool,
    pub start: Option<Start>,
    pub comments: Vec<String>,
}

#[derive(Debug)]
pub struct Variant {
    pub uri: String,
    pub bandwidth: u64,
    pub average_bandwidth: Option<u64>,
    pub codecs: String,
    pub resolution: Option<Resolution>,
    pub frame_rate: Option<f64>,
    pub hdcp_level: Option<HDCPLevel>,
    pub audio: Option<GroupId>,
    pub video: Option<GroupId>,
    pub subtitles: Option<GroupId>,
    pub closed_captions: Option<GroupId>,
}

#[derive(Debug)]
pub struct IFrameVariant {
    pub bandwidth: u64,
    pub average_bandwidth: Option<u64>,
    pub codecs: Option<String>,
    pub resolution: Option<Resolution>,
    pub hdcp_level: Option<HDCPLevel>,
    pub video: Option<GroupId>,
    pub uri: String,
}

#[derive(Debug)]
pub struct Rendition {
    pub media_type: MediaType,
    pub uri: Option<String>,
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

#[derive(Debug)]
pub enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

#[derive(Debug)]
pub struct Resolution {
    pub width: u64,
    pub height: u64,
}

#[derive(Debug)]
pub enum HDCPLevel {
    Type0,
}

#[derive(Debug)]
pub struct SessionData {
    pub data_id: String,
    pub entry: SessionDataEntry,
    pub language: Option<String>,
}

#[derive(Debug)]
pub enum SessionDataEntry {
    Value(String),
    Uri(String),
}

#[derive(Debug)]
pub struct SessionKey {
    pub method: SessionKeyMethod,
    pub uri: String,
    pub iv: Option<u128>,
    pub key_format: Option<String>,
    pub key_format_versions: Option<String>,
}

#[derive(Debug)]
pub enum SessionKeyMethod {
    AES128,
    SampleAES,
}

#[derive(Debug)]
pub struct Start {
    pub time_offset: f64,
    pub precise: bool,
}

#[derive(Debug)]
pub enum MasterPlaylistParseError {
    /// This error denotes that the EXTM3U tag is missing from the begining of the playlist.
    MissingExtm3u,

    /// This error denotes that there is a duplicate tag that is not allowed.
    DuplicateTag,

    /// This error denotes that a tag's configuration is invalid.
    InvalidTag,

    /// This error denotes that a tag is missing required data.
    IncompleteTag,

    /// This error denotes that a tag is incompatible with the playlist type.
    IncompatibleTag,
}

enum Line {
    Tag(Tags),
    Comment(String),
    Uri(String),
    Empty,
}

impl FromStr for MasterPlaylist {
    type Err = MasterPlaylistParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = separated_list0(
            line_ending,
            alt((
                preceded(
                    char('#'),
                    alt((
                        map(alt(TAG_PARSERS), |tag| Line::Tag(tag)),
                        map(not_line_ending, |comment: &str| {
                            Line::Comment(comment.to_string())
                        }),
                    )),
                ),
                map(not_line_ending, |s: &str| {
                    if s.is_empty() {
                        Line::Empty
                    } else {
                        Line::Uri(s.to_string())
                    }
                }),
            )),
        )
        .parse(s)
        .unwrap()
        .1
        .into_iter();

        let mut version = None;
        let mut variants = Vec::new();
        let mut i_frame_variants = Vec::new();
        let mut renditions = Vec::new();
        let mut session_data = Vec::new();
        let mut session_keys = Vec::new();
        let mut independent_segments = None;
        let mut start = None;
        let mut comments = Vec::new();

        // Check for EXTM3U tag at first line
        if !matches!(
            lines.next(),
            Some(Line::Tag(Tags::Basic(BasicTags::Extm3u)))
        ) {
            return Err(MasterPlaylistParseError::MissingExtm3u);
        }

        while let Some(line) = lines.next() {
            match line {
                Line::Tag(tag) => match tag {
                    Tags::Basic(BasicTags::Extm3u) => {
                        return Err(MasterPlaylistParseError::DuplicateTag);
                    }
                    Tags::Basic(BasicTags::ExtXVersion(v)) => {
                        if version.is_some() {
                            return Err(MasterPlaylistParseError::DuplicateTag);
                        }
                        version = Some(v);
                    }
                    Tags::MediaSegment(_) | Tags::MediaPlaylist(_) => {
                        return Err(MasterPlaylistParseError::IncompatibleTag);
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXMedia(v)) => {
                        let media_type = match v.media_type {
                            ExtXMediaMediaType::Audio => MediaType::Audio,
                            ExtXMediaMediaType::Video => MediaType::Video,
                            ExtXMediaMediaType::Subtitles => MediaType::Subtitles,
                            ExtXMediaMediaType::ClosedCaptions => MediaType::ClosedCaptions,
                        };

                        renditions.push(Rendition {
                            media_type,
                            uri: v.uri,
                            group_id: v.group_id,
                            language: v.language,
                            associated_language: v.associated_language,
                            name: v.name,
                            default: v.default,
                            autoselect: v.autoselect,
                            forced: v.forced,
                            instream_id: v.instream_id,
                            characteristics: v.characteristics,
                            channels: v.channels,
                        });
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXStreamInf(v)) => {
                        if let Some(Line::Uri(uri)) = lines.next() {
                            let resolution = v.resolution.map(|v| Resolution {
                                height: v.height,
                                width: v.width,
                            });
                            let hdcp_level = v.hdcp_level.map(|v| match v {
                                ExtXStreamInfHDCPLevel::Type0 => HDCPLevel::Type0,
                            });

                            variants.push(Variant {
                                uri,
                                bandwidth: v.bandwidth,
                                average_bandwidth: v.average_bandwidth,
                                codecs: v.codecs,
                                resolution,
                                frame_rate: v.frame_rate,
                                hdcp_level,
                                audio: v.audio,
                                video: v.video,
                                subtitles: v.subtitles,
                                closed_captions: v.closed_captions,
                            });
                        } else {
                            return Err(MasterPlaylistParseError::IncompleteTag);
                        }
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXIFrameStreamInf(v)) => {
                        let resolution = v.resolution.map(|v| Resolution {
                            height: v.height,
                            width: v.width,
                        });
                        let hdcp_level = v.hdcp_level.map(|v| match v {
                            ExtXIFrameStreamInfHDCPLevel::Type0 => HDCPLevel::Type0,
                        });

                        i_frame_variants.push(IFrameVariant {
                            bandwidth: v.bandwidth,
                            average_bandwidth: v.average_bandwidth,
                            codecs: v.codecs,
                            resolution,
                            hdcp_level,
                            video: v.video,
                            uri: v.uri,
                        });
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXSessionData(v)) => {
                        let entry = match v.entry {
                            ExtXSessionDataEntry::Value(v) => SessionDataEntry::Value(v),
                            ExtXSessionDataEntry::Uri(v) => SessionDataEntry::Uri(v),
                        };

                        session_data.push(SessionData {
                            data_id: v.data_id,
                            entry,
                            language: v.language,
                        });
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXSessionKey(v)) => {
                        let method = match v.method {
                            ExtXSessionKeyMethod::AES128 => SessionKeyMethod::AES128,
                            ExtXSessionKeyMethod::SampleAES => SessionKeyMethod::SampleAES,
                        };

                        session_keys.push(SessionKey {
                            method,
                            uri: v.uri,
                            iv: v.iv,
                            key_format: v.key_format,
                            key_format_versions: v.key_format_versions,
                        });
                    }
                    Tags::Playlist(PlaylistTags::ExtXIndependentSegments) => {
                        if independent_segments.is_some() {
                            return Err(MasterPlaylistParseError::DuplicateTag);
                        }
                        independent_segments = Some(true);
                    }
                    Tags::Playlist(PlaylistTags::ExtXStart(v)) => {
                        if start.is_some() {
                            return Err(MasterPlaylistParseError::DuplicateTag);
                        }
                        start = Some(Start {
                            time_offset: v.time_offset,
                            precise: v.precise,
                        });
                    }
                },
                Line::Comment(comment) => {
                    comments.push(comment);
                }
                _ => {}
            }
        }

        Ok(Self {
            version: version.unwrap_or(Version::V1),
            variants,
            i_frame_variants,
            renditions,
            session_data,
            session_keys,
            independent_segments: independent_segments.unwrap_or(false),
            start,
            comments,
        })
    }
}
