use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{ExtTag, MediaPlaylistTag, Tag},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXIFramesOnly;

impl ExtTag for ExtXIFramesOnly {
    const TAG_PREFIX: &'static str = "EXT-X-I-FRAMES-ONLY";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tag::MediaPlaylist(MediaPlaylistTag::ExtXIFramesOnly)
        })
        .parse(s)
    }
}
