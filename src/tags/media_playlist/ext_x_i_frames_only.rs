use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{MediaPlaylistTags, Tag, Tags},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXIFramesOnly;

impl Tag for ExtXIFramesOnly {
    const TAG_PREFIX: &'static str = "EXT-X-I-FRAMES-ONLY";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tags::MediaPlaylist(MediaPlaylistTags::ExtXIFramesOnly)
        })
        .parse(s)
    }
}
