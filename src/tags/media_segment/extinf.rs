use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, not_line_ending},
    combinator::map,
    number::complete::double,
    sequence::separated_pair,
};

use crate::{
    tags::{MediaSegmentTags, Tag, Tags},
    types::Version,
};

#[derive(Debug)]
pub struct Extinf {
    pub duration: f64,
    pub title: Option<String>,
}

impl Tag for Extinf {
    const TAG_PREFIX: &'static str = "EXTINF";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
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
                Tags::MediaSegment(MediaSegmentTags::Extinf(Self { duration, title }))
            },
        )
        .parse(s)
    }
}
