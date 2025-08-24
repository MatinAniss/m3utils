use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, not_line_ending},
    combinator::map,
    number::complete::double,
    sequence::separated_pair,
};

use crate::{
    tags::{MediaSegmentTag, ExtTag, Tag},
    types::Version,
};

#[derive(Debug)]
pub struct Extinf {
    pub duration: f64,
    pub title: Option<String>,
}

impl ExtTag for Extinf {
    const TAG_PREFIX: &'static str = "EXTINF";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(
            separated_pair(
                tag(Self::TAG_PREFIX),
                char(':'),
                separated_pair(
                    double,
                    char(','),
                    map(not_line_ending, |s: &str| {
                        if s.is_empty() {
                            None
                        } else {
                            Some(s.to_string())
                        }
                    }),
                ),
            ),
            |(_, (duration, title))| {
                Tag::MediaSegment(MediaSegmentTag::Extinf(Self { duration, title }))
            },
        )
        .parse(s)
    }
}
