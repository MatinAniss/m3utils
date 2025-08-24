use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::char, combinator::map,
    sequence::separated_pair,
};

use crate::{
    tags::{MediaSegmentTag, ExtTag, Tag, utils::date_time},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXProgramDateTime;

impl ExtTag for ExtXProgramDateTime {
    const TAG_PREFIX: &'static str = "EXT-X-PROGRAM-DATE-TIME";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(
            separated_pair(tag(Self::TAG_PREFIX), char(':'), date_time),
            |(_, date_time)| Tag::MediaSegment(MediaSegmentTag::ExtXProgramDateTime(date_time)),
        )
        .parse(s)
    }
}
