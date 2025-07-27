use nom::IResult;

use crate::{
    tags::{
        basic::{BasicTags, ext_x_version::ExtXVersion, extm3u::Extm3u},
        master_playlist::{
            MasterPlaylistTags, ext_x_i_frame_stream_inf::ExtXIFrameStreamInf,
            ext_x_media::ExtXMedia, ext_x_session_data::ExtXSessionData,
            ext_x_session_key::ExtXSessionKey, ext_x_stream_inf::ExtXStreamInf,
        },
        media_playlist::MediaPlaylistTags,
        media_segment::MediaSegmentTags,
        playlist::{
            PlaylistTags, ext_x_independent_segments::ExtXIndependentSegments,
            ext_x_start::ExtXStart,
        },
    },
    version::Version,
};

pub(crate) mod basic;
pub(crate) mod master_playlist;
pub(crate) mod media_playlist;
pub(crate) mod media_segment;
pub(crate) mod playlist;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Tags {
    Basic(BasicTags),
    MasterPlaylist(MasterPlaylistTags),
    MediaPlaylist(MediaPlaylistTags),
    MediaSegment(MediaSegmentTags),
    Playlist(PlaylistTags),
}

#[allow(dead_code)]
pub(crate) trait Tag {
    const TAG_PREFIX: &'static str;

    fn min_version() -> Version;

    fn parse(s: &str) -> IResult<&str, Tags>;
}

pub(crate) const TAG_PARSERS: [fn(&str) -> IResult<&str, Tags>; 9] = [
    // Basic Tags
    Extm3u::parse,
    ExtXVersion::parse,
    // Media Segment Tags
    // todo
    // Media Playlist Tags
    // todo
    // Master Playlist Tags
    ExtXMedia::parse,
    ExtXStreamInf::parse,
    ExtXIFrameStreamInf::parse,
    ExtXSessionData::parse,
    ExtXSessionKey::parse,
    // Playlist Tags
    ExtXIndependentSegments::parse,
    ExtXStart::parse,
];
