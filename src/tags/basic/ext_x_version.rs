use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, u8},
    combinator::map,
    sequence::separated_pair,
};

use crate::{
    tags::{Tag, Tags, basic::BasicTags},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXVersion;

impl Tag for ExtXVersion {
    const TAG_PREFIX: &'static str = "EXT-X-VERSION";

    fn min_version() -> Version {
        Version::V1
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(
            separated_pair(tag(Self::TAG_PREFIX), char(':'), u8),
            |(_, version)| Tags::Basic(BasicTags::ExtXVersion(version.into())),
        )
        .parse(s)
    }
}
