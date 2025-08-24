use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::{map, opt},
    sequence::{pair, preceded, separated_pair},
};

use crate::{
    tags::{MediaSegmentTags, Tag, Tags},
    types::{ByteRange, Version},
};

#[derive(Debug)]
pub struct ExtXByterange;

impl Tag for ExtXByterange {
    const TAG_PREFIX: &'static str = "EXT-X-BYTERANGE";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(
            separated_pair(
                tag(Self::TAG_PREFIX),
                char(':'),
                pair(u64, opt(preceded(char('@'), u64))),
            ),
            |(_, (length, offset))| {
                Tags::MediaSegment(MediaSegmentTags::ExtXByterange(ByteRange {
                    length,
                    offset,
                }))
            },
        )
        .parse(s)
    }
}
