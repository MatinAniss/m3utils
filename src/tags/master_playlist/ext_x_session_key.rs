use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, hex_digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
};

use crate::{
    tags::{Tag, Tags, master_playlist::MasterPlaylistTags},
    version::Version,
};

#[derive(Debug)]
pub(crate) struct ExtXSessionKey {
    pub(crate) method: ExtXSessionKeyMethod,
    pub(crate) uri: String,
    pub(crate) iv: Option<u128>,
    pub(crate) key_format: Option<String>,
    pub(crate) key_format_versions: Option<String>,
}

#[derive(Debug)]
pub(crate) enum ExtXSessionKeyMethod {
    AES128,
    SampleAES,
}

enum ExtXSessionKeyAttributes {
    Method(ExtXSessionKeyMethod),
    Uri(String),
    Iv(u128),
    KeyFormat(String),
    KeyFormatVersions(String),
    Unknown,
}

impl Tag for ExtXSessionKey {
    const TAG_PREFIX: &'static str = "EXT-X-SESSION-KEY";

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
                                tag("METHOD"),
                                char('='),
                                alt((
                                    map(tag("AES-128"), |_| ExtXSessionKeyMethod::AES128),
                                    map(tag("SAMPLE-AES"), |_| ExtXSessionKeyMethod::SampleAES),
                                )),
                            ),
                            |(_, method)| ExtXSessionKeyAttributes::Method(method),
                        ),
                        map(
                            separated_pair(
                                tag("URI"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, uri)| ExtXSessionKeyAttributes::Uri(uri.to_string()),
                        ),
                        map_res(
                            separated_pair(
                                tag("IV"),
                                char('='),
                                preceded(alt((tag("0x"), tag("0X"))), hex_digit1),
                            ),
                            |(_, iv)| {
                                u128::from_str_radix(iv, 16)
                                    .map(|iv| ExtXSessionKeyAttributes::Iv(iv))
                            },
                        ),
                        map(
                            separated_pair(
                                tag("KEYFORMAT"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, key_format)| {
                                ExtXSessionKeyAttributes::KeyFormat(key_format.to_string())
                            },
                        ),
                        map(
                            separated_pair(
                                tag("KEYFORMATVERSIONS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, key_format_versions)| {
                                ExtXSessionKeyAttributes::KeyFormatVersions(
                                    key_format_versions.to_string(),
                                )
                            },
                        ),
                        map(not_line_ending_or_comma, |_| {
                            ExtXSessionKeyAttributes::Unknown
                        }),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut method = None;
                let mut uri = None;
                let mut iv = None;
                let mut key_format = None;
                let mut key_format_versions = None;

                for attribute in attributes {
                    match attribute {
                        ExtXSessionKeyAttributes::Method(v) => {
                            if method.is_some() {
                                return Err("");
                            }
                            method = Some(v);
                        }
                        ExtXSessionKeyAttributes::Uri(v) => {
                            if uri.is_some() {
                                return Err("");
                            }
                            uri = Some(v);
                        }
                        ExtXSessionKeyAttributes::Iv(v) => {
                            if iv.is_some() {
                                return Err("");
                            }
                            iv = Some(v);
                        }
                        ExtXSessionKeyAttributes::KeyFormat(v) => {
                            if key_format.is_some() {
                                return Err("");
                            }
                            key_format = Some(v);
                        }
                        ExtXSessionKeyAttributes::KeyFormatVersions(v) => {
                            if key_format_versions.is_some() {
                                return Err("");
                            }
                            key_format_versions = Some(v);
                        }
                        ExtXSessionKeyAttributes::Unknown => {}
                    }
                }

                Ok(Tags::MasterPlaylist(MasterPlaylistTags::ExtXSessionKey(
                    Self {
                        method: method.ok_or("")?,
                        uri: uri.ok_or("")?,
                        iv,
                        key_format,
                        key_format_versions,
                    },
                )))
            },
        )
        .parse(s)
    }
}

fn not_line_ending_or_comma(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '\n' || c == '\r' || c == ',').parse(s)
}

fn not_quote(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '"').parse(s)
}
