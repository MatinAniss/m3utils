use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::map,
    sequence::separated_pair,
};

use crate::{
    tags::{MediaPlaylistTags, Tag, Tags},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXTargetduration;

impl Tag for ExtXTargetduration {
    const TAG_PREFIX: &'static str = "EXT-X-TARGETDURATION";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(
            separated_pair(tag(Self::TAG_PREFIX), char(':'), u64),
            |(_, duration)| Tags::MediaPlaylist(MediaPlaylistTags::ExtXTargetduration(duration)),
        )
        .parse(s)
    }
}
