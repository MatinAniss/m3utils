use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::map,
    sequence::separated_pair,
};

use crate::{
    tags::{MediaPlaylistTag, ExtTag, Tag},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXTargetduration;

impl ExtTag for ExtXTargetduration {
    const TAG_PREFIX: &'static str = "EXT-X-TARGETDURATION";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(
            separated_pair(tag(Self::TAG_PREFIX), char(':'), u64),
            |(_, duration)| Tag::MediaPlaylist(MediaPlaylistTag::ExtXTargetduration(duration)),
        )
        .parse(s)
    }
}
