use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{Tag, Tags, playlist::PlaylistTags},
    version::Version,
};

#[derive(Debug)]
pub(crate) struct ExtXIndependentSegments;

impl Tag for ExtXIndependentSegments {
    const TAG_PREFIX: &'static str = "EXT-X-INDEPENDENT-SEGMENTS";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tags::Playlist(PlaylistTags::ExtXIndependentSegments)
        })
        .parse(s)
    }
}
