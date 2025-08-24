use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{MediaSegmentTags, Tag, Tags},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXDiscontinuity {
    pub length: u64,
    pub offset: Option<u64>,
}

impl Tag for ExtXDiscontinuity {
    const TAG_PREFIX: &'static str = "EXT-X-DISCONTINUITY";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tags::MediaSegment(MediaSegmentTags::ExtXDiscontinuity)
        })
        .parse(s)
    }
}
