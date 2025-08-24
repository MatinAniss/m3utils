use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{ExtTag, Tag, playlist::PlaylistTag},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXIndependentSegments;

impl ExtTag for ExtXIndependentSegments {
    const TAG_PREFIX: &'static str = "EXT-X-INDEPENDENT-SEGMENTS";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tag::Playlist(PlaylistTag::ExtXIndependentSegments)
        })
        .parse(s)
    }
}
