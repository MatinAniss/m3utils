use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{Tag, Tags, basic::BasicTags},
    version::Version,
};

#[derive(Debug)]
pub(crate) struct Extm3u;

impl Tag for Extm3u {
    const TAG_PREFIX: &'static str = "EXTM3U";

    fn min_version() -> Version {
        Version::V1
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(tag(Self::TAG_PREFIX), |_| Tags::Basic(BasicTags::Extm3u)).parse(s)
    }
}
