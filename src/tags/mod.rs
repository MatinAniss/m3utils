mod basic;
mod master_playlist;
mod media_playlist;
mod media_segment;
mod playlist;
mod utils;

use nom::IResult;

pub use basic::*;
pub use master_playlist::*;
pub use media_playlist::*;
pub use media_segment::*;
pub use playlist::*;

use crate::types::Version;

#[derive(Debug)]
pub enum Tag {
    Basic(BasicTag),
    MasterPlaylist(MasterPlaylistTag),
    MediaPlaylist(MediaPlaylistTag),
    MediaSegment(MediaSegmentTag),
    Playlist(PlaylistTag),
}

pub(crate) trait ExtTag {
    const TAG_PREFIX: &'static str;

    #[allow(dead_code)]
    fn min_version() -> Version;

    fn parse(s: &str) -> IResult<&str, Tag>;
}

pub(crate) const TAG_PARSERS: [fn(&str) -> IResult<&str, Tag>; 22] = [
    // Basic Tags
    Extm3u::parse,
    ExtXVersion::parse,
    // Media Segment Tags
    Extinf::parse,
    ExtXByterange::parse,
    ExtXDiscontinuity::parse,
    ExtXKey::parse,
    ExtXMap::parse,
    ExtXProgramDateTime::parse,
    ExtXDaterange::parse,
    // Media Playlist Tags
    ExtXTargetduration::parse,
    ExtXMediaSequence::parse,
    ExtXDiscontinuitySequence::parse,
    ExtXEndlist::parse,
    ExtXPlaylistType::parse,
    ExtXIFramesOnly::parse,
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
