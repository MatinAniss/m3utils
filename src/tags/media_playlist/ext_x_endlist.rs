use nom::{IResult, Parser, bytes::complete::tag, combinator::map};

use crate::{
    tags::{ExtTag, MediaPlaylistTag, Tag},
    types::Version,
};

#[derive(Debug)]
pub struct ExtXEndlist;

impl ExtTag for ExtXEndlist {
    const TAG_PREFIX: &'static str = "EXT-X-ENDLIST";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(tag(Self::TAG_PREFIX), |_| {
            Tag::MediaPlaylist(MediaPlaylistTag::ExtXEndlist)
        })
        .parse(s)
    }
}
