use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::map,
    sequence::separated_pair,
};

use crate::{
    tags::{ExtTag, MediaPlaylistTag, Tag},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXDiscontinuitySequence;

impl ExtTag for ExtXDiscontinuitySequence {
    const TAG_PREFIX: &'static str = "EXT-X-DISCONTINUITY-SEQUENCE";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(
            separated_pair(tag(Self::TAG_PREFIX), char(':'), u64),
            |(_, discontinuity_sequence)| {
                Tag::MediaPlaylist(MediaPlaylistTag::ExtXDiscontinuitySequence(
                    discontinuity_sequence,
                ))
            },
        )
        .parse(s)
    }
}
