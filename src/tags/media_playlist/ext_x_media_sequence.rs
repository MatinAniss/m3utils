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
pub struct ExtXMediaSequence;

impl Tag for ExtXMediaSequence {
    const TAG_PREFIX: &'static str = "EXT-X-MEDIA-SEQUENCE";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(
            separated_pair(tag(Self::TAG_PREFIX), char(':'), u64),
            |(_, sequence)| Tags::MediaPlaylist(MediaPlaylistTags::ExtXMediaSequence(sequence)),
        )
        .parse(s)
    }
}
