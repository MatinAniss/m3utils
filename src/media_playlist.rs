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
    tags::{
        BasicTags, Extinf, MediaPlaylistTags, MediaSegmentTags, PlaylistTags, TAG_PARSERS, Tags,
    },
    types::{MediaPlaylistType, Segment, Start, Version},
};

#[derive(Debug)]
pub struct MediaPlaylist {
    pub version: Version,
    pub segments: Vec<Segment>,
    pub target_duration: u64,
    pub media_sequence: u64,
    pub discontinuity_sequence: u64,
    pub end_list: bool,
    pub playlist_type: Option<MediaPlaylistType>,
    pub i_frames_only: bool,
    pub independent_segments: bool,
    pub start: Option<Start>,
    pub comments: Vec<String>,
}

#[derive(Debug)]
enum Line {
    Tag(Tags),
    Comment(String),
    Uri(String),
    Empty,
}

impl FromStr for MediaPlaylist {
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
        let mut segments = Vec::new();
        let mut target_duration = None;
        let mut media_sequence = None;
        let mut discontinuity_sequence = None;
        let mut end_list = None;
        let mut playlist_type = None;
        let mut i_frames_only = None;
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

        // Preprocess media segment tags
        let mut extinf = None;
        let mut ext_x_byterange = None;
        let mut ext_x_discontinuity = None;
        let mut ext_x_key = None;
        let mut ext_x_map = None;
        let mut ext_x_program_date_time = None;
        let mut ext_x_daterange = None;

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
                    Tags::MediaSegment(MediaSegmentTags::Extinf(v)) => {
                        if extinf.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        extinf = Some(v);
                    }
                    Tags::MediaSegment(MediaSegmentTags::ExtXByterange(v)) => {
                        if ext_x_byterange.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        ext_x_byterange = Some(v);
                    }
                    Tags::MediaSegment(MediaSegmentTags::ExtXDiscontinuity) => {
                        if ext_x_discontinuity.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        ext_x_discontinuity = Some(true);
                    }
                    Tags::MediaSegment(MediaSegmentTags::ExtXKey(v)) => {
                        if ext_x_key.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        ext_x_key = Some(v);
                    }
                    Tags::MediaSegment(MediaSegmentTags::ExtXMap(v)) => {
                        if ext_x_map.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        ext_x_map = Some(v);
                    }
                    Tags::MediaSegment(MediaSegmentTags::ExtXProgramDateTime(v)) => {
                        if ext_x_program_date_time.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        ext_x_program_date_time = Some(v);
                    }
                    Tags::MediaSegment(MediaSegmentTags::ExtXDaterange(v)) => {
                        if ext_x_daterange.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        ext_x_daterange = Some(v);
                    }
                    Tags::MediaPlaylist(MediaPlaylistTags::ExtXTargetduration(v)) => {
                        if target_duration.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        target_duration = Some(v);
                    }
                    Tags::MediaPlaylist(MediaPlaylistTags::ExtXMediaSequence(v)) => {
                        if media_sequence.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        media_sequence = Some(v);
                    }
                    Tags::MediaPlaylist(MediaPlaylistTags::ExtXDiscontinuitySequence(v)) => {
                        if discontinuity_sequence.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        discontinuity_sequence = Some(v);
                    }
                    Tags::MediaPlaylist(MediaPlaylistTags::ExtXEndlist) => {
                        if end_list.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        end_list = Some(true);
                    }
                    Tags::MediaPlaylist(MediaPlaylistTags::ExtXPlaylistType(v)) => {
                        if playlist_type.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        playlist_type = Some(v);
                    }
                    Tags::MediaPlaylist(MediaPlaylistTags::ExtXIFramesOnly) => {
                        if i_frames_only.is_some() {
                            return Err(Error::DuplicateTag);
                        }
                        i_frames_only = Some(true);
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
                    Tags::MasterPlaylist(_) => {
                        return Err(Error::IncompatibleTag);
                    }
                },
                Line::Uri(uri) => {
                    let Extinf { title, duration } = extinf.take().ok_or(Error::MissingTag)?;
                    let byte_range = ext_x_byterange.take();
                    let discontinuity = ext_x_discontinuity.take().unwrap_or(false);
                    let key = ext_x_key.take();
                    let map = ext_x_map.take();
                    let program_date_time = ext_x_program_date_time.take();
                    let date_range = ext_x_daterange.take();

                    segments.push(Segment {
                        uri,
                        duration,
                        title,
                        byte_range,
                        discontinuity,
                        key,
                        map,
                        program_date_time,
                        date_range,
                    });
                }
                Line::Comment(comment) => {
                    comments.push(comment);
                }
                Line::Empty => {}
            }
        }

        Ok(Self {
            version: version.unwrap_or(Version::V1),
            segments,
            target_duration: target_duration.ok_or(Error::MissingTag)?,
            media_sequence: media_sequence.unwrap_or(0),
            discontinuity_sequence: discontinuity_sequence.unwrap_or(0),
            end_list: end_list.unwrap_or(false),
            playlist_type,
            i_frames_only: i_frames_only.unwrap_or(false),
            independent_segments: independent_segments.unwrap_or(false),
            start,
            comments,
        })
    }
}
