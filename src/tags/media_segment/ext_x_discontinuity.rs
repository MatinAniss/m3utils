use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{MediaSegmentTag, ExtTag, Tag},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXDiscontinuity {
    pub length: u64,
    pub offset: Option<u64>,
}

impl ExtTag for ExtXDiscontinuity {
    const TAG_PREFIX: &'static str = "EXT-X-DISCONTINUITY";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tag::MediaSegment(MediaSegmentTag::ExtXDiscontinuity)
        })
        .parse(s)
    }
}
