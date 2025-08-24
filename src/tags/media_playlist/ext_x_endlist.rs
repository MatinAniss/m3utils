use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{MediaPlaylistTags, Tag, Tags},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXEndlist;

impl Tag for ExtXEndlist {
    const TAG_PREFIX: &'static str = "EXT-X-ENDLIST";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tags::MediaPlaylist(MediaPlaylistTags::ExtXEndlist)
        })
        .parse(s)
    }
}
