use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{ExtTag, Tag, basic::BasicTag},
    types::Version,
};

#[derive(Debug)]
pub struct Extm3u;

impl ExtTag for Extm3u {
    const TAG_PREFIX: &'static str = "EXTM3U";

    fn min_version() -> Version {
        Version::V1
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(tag(Self::TAG_PREFIX), |_| Tag::Basic(BasicTag::Extm3u)).parse(s)
    }
}
