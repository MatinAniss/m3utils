use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::char,
    combinator::{map, map_res},
    multi::separated_list0,
    number::complete::double,
    sequence::separated_pair,
};

use crate::{
    tags::{Tag, Tags, playlist::PlaylistTags},
    types::{Start, Version},
};

#[derive(Debug)]
pub struct ExtXStart;

enum ExtXStartAttributes {
    TimeOffset(f64),
    Precise(bool),
    Unknown,
}

impl Tag for ExtXStart {
    const TAG_PREFIX: &'static str = "EXT-X-START";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tags> {
        map_res(
            separated_pair(
                tag(Self::TAG_PREFIX),
                char(':'),
                separated_list0(
                    char(','),
                    alt((
                        map(
                            separated_pair(tag("TIME-OFFSET"), char('='), double),
                            |(_, time_offset)| ExtXStartAttributes::TimeOffset(time_offset),
                        ),
                        map(
                            separated_pair(
                                tag("PRECISE"),
                                char('='),
                                alt((map(tag("YES"), |_| true), map(tag("NO"), |_| false))),
                            ),
                            |(_, precise)| ExtXStartAttributes::Precise(precise),
                        ),
                        map(not_line_ending_or_comma, |_| ExtXStartAttributes::Unknown),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut time_offset = None;
                let mut precise = None;

                for attribute in attributes {
                    match attribute {
                        ExtXStartAttributes::TimeOffset(v) => {
                            if time_offset.is_some() {
                                return Err("");
                            }
                            time_offset = Some(v);
                        }
                        ExtXStartAttributes::Precise(v) => {
                            if precise.is_some() {
                                return Err("");
                            }
                            precise = Some(v);
                        }
                        ExtXStartAttributes::Unknown => {}
                    }
                }

                Ok(Tags::Playlist(PlaylistTags::ExtXStart(Start {
                    time_offset: time_offset.ok_or("")?,
                    precise: precise.unwrap_or(false),
                })))
            },
        )
        .parse(s)
    }
}

fn not_line_ending_or_comma(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '\n' || c == '\r' || c == ',').parse(s)
}
