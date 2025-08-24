use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::{map, map_res, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair},
};

use crate::{
    tags::{
        MediaSegmentTags, Tag, Tags,
        utils::{not_line_ending_or_comma, not_quote},
    },
    types::{ByteRange, Map, Version},
};

#[derive(Debug)]
pub struct ExtXMap {
    pub uri: String,
    pub byterange: Option<ByteRange>,
}

enum ExtXMapAttributes {
    Uri(String),
    Byterange(ByteRange),
    Unknown,
}

impl Tag for ExtXMap {
    const TAG_PREFIX: &'static str = "EXT-X-MAP";

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
                            separated_pair(
                                tag("URI"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, uri)| ExtXMapAttributes::Uri(uri.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("BYTERANGE"),
                                char('='),
                                pair(u64, opt(preceded(char('@'), u64))),
                            ),
                            |(_, (length, offset))| {
                                ExtXMapAttributes::Byterange(ByteRange { length, offset })
                            },
                        ),
                        map(not_line_ending_or_comma, |_| ExtXMapAttributes::Unknown),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut uri = None;
                let mut byterange = None;

                for attribute in attributes {
                    match attribute {
                        ExtXMapAttributes::Uri(v) => {
                            if uri.is_some() {
                                return Err("");
                            }
                            uri = Some(v);
                        }
                        ExtXMapAttributes::Byterange(v) => {
                            if byterange.is_some() {
                                return Err("");
                            }
                            byterange = Some(v);
                        }
                        ExtXMapAttributes::Unknown => {}
                    }
                }

                Ok(Tags::MediaSegment(MediaSegmentTags::ExtXMap(Map {
                    uri: uri.ok_or("")?,
                    byterange,
                })))
            },
        )
        .parse(s)
    }
}
