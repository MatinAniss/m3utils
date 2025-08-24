use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::char, combinator::map,
    sequence::separated_pair,
};

use crate::{
    tags::{MediaPlaylistTag, ExtTag, Tag},
    types::{MediaPlaylistType, Version},
};

#[derive(Debug)]
pub struct ExtXPlaylistType;

impl ExtTag for ExtXPlaylistType {
    const TAG_PREFIX: &'static str = "EXT-X-PLAYLIST-TYPE";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
        map(
            separated_pair(
                tag(Self::TAG_PREFIX),
                char(':'),
                alt((
                    map(tag("EVENT"), |_| MediaPlaylistType::Event),
                    map(tag("VOD"), |_| MediaPlaylistType::Vod),
                )),
            ),
            |(_, media_playlist_type)| {
                Tag::MediaPlaylist(MediaPlaylistTag::ExtXPlaylistType(media_playlist_type))
            },
        )
        .parse(s)
    }
}
