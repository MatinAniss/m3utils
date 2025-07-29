mod basic;
mod master_playlist;
mod media_playlist;
mod media_segment;
mod playlist;

use nom::IResult;

pub use basic::*;
pub use master_playlist::*;
pub use media_playlist::*;
pub use media_segment::*;
pub use playlist::*;

use crate::types::Version;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Tags {
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
