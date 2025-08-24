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
    Error,
    tags::{BasicTags, MasterPlaylistTags, PlaylistTags, TAG_PARSERS, Tags},
    types::{Data, DataEntry, IFrameVariant, Key, Rendition, Start, Variant, Version},
};

#[derive(Debug)]
pub struct MasterPlaylist {
    pub version: Version,
    pub renditions: Vec<Rendition>,
    pub variants: Vec<Variant>,
    pub i_frame_variants: Vec<IFrameVariant>,
    pub session_data: Vec<Data>,
    pub session_keys: Vec<Key>,
    pub independent_segments: bool,
    pub start: Option<Start>,
    pub comments: Vec<String>,
}

enum Line {
    Tag(Tags),
    Comment(String),
    Uri(String),
    Empty,
}

impl FromStr for MasterPlaylist {
    type Err = Error;

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
        let mut renditions = Vec::new();
        let mut variants = Vec::new();
        let mut i_frame_variants = Vec::new();
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
            return Err(Error::MissingExtm3u);
        }

        while let Some(line) = lines.next() {
            match line {
                Line::Tag(tag) => match tag {
                    Tags::Basic(BasicTags::Extm3u) => {
                        return Err(Error::DuplicateTag);
                    }
                    Tags::Basic(BasicTags::ExtXVersion(v)) => {
                        if version.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        version = Some(v);
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXMedia(v)) => {
                        renditions.push(Rendition {
                            media_type: v.media_type,
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
                            variants.push(Variant {
                                uri,
                                bandwidth: v.bandwidth,
                                average_bandwidth: v.average_bandwidth,
                                codecs: v.codecs,
                                resolution: v.resolution,
                                frame_rate: v.frame_rate,
                                hdcp_level: v.hdcp_level,
                                audio: v.audio,
                                video: v.video,
                                subtitles: v.subtitles,
                                closed_captions: v.closed_captions,
                            });
                        } else {
                            return Err(Error::IncompleteTag);
                        }
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXIFrameStreamInf(v)) => {
                        i_frame_variants.push(IFrameVariant {
                            bandwidth: v.bandwidth,
                            average_bandwidth: v.average_bandwidth,
                            codecs: v.codecs,
                            resolution: v.resolution,
                            hdcp_level: v.hdcp_level,
                            video: v.video,
                            uri: v.uri,
                        });
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXSessionData(v)) => {
                        let entry = match (v.value, v.uri) {
                            (Some(_), Some(_)) => {
                                return Err(Error::InvalidTag);
                            }
                            (Some(value), None) => DataEntry::Value(value),
                            (None, Some(uri)) => DataEntry::Uri(uri),
                            (None, None) => {
                                return Err(Error::IncompleteTag);
                            }
                        };

                        session_data.push(Data {
                            data_id: v.data_id,
                            entry,
                            language: v.language,
                        });
                    }
                    Tags::MasterPlaylist(MasterPlaylistTags::ExtXSessionKey(v)) => {
                        session_keys.push(Key {
                            method: v.method,
                            uri: v.uri,
                            iv: v.iv,
                            key_format: v.key_format,
                            key_format_versions: v.key_format_versions,
                        });
                    }
                    Tags::Playlist(PlaylistTags::ExtXIndependentSegments) => {
                        if independent_segments.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        independent_segments = Some(true);
                    }
                    Tags::Playlist(PlaylistTags::ExtXStart(v)) => {
                        if start.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        start = Some(Start {
                            time_offset: v.time_offset,
                            precise: v.precise,
                        });
                    }
                    Tags::MediaSegment(_) | Tags::MediaPlaylist(_) => {
                        return Err(Error::IncompatibleTag);
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
            renditions,
            variants,
            i_frame_variants,
            session_data,
            session_keys,
            independent_segments: independent_segments.unwrap_or(false),
            start,
            comments,
        })
    }
}
