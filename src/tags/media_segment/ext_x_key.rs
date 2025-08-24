use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};

use crate::{
    tags::{
        MediaSegmentTag, ExtTag, Tag,
        utils::{hexadecimal, not_line_ending_or_comma, not_quote},
    },
    types::{EncryptionMethod, Key, Version},
};

#[derive(Debug)]
pub struct ExtXKey {
    pub method: EncryptionMethod,
    pub uri: String,
    pub iv: Option<u128>,
    pub key_format: Option<String>,
    pub key_format_versions: Option<String>,
}

enum ExtXKeyAttributes {
    Method(EncryptionMethod),
    Uri(String),
    Iv(u128),
    KeyFormat(String),
    KeyFormatVersions(String),
    Unknown,
}

impl ExtTag for ExtXKey {
    const TAG_PREFIX: &'static str = "EXT-X-KEY";

    fn min_version() -> Version {
        todo!()
    }

    fn parse(s: &str) -> IResult<&str, Tag> {
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
                                    map(tag("NONE"), |_| EncryptionMethod::None),
                                    map(tag("AES-128"), |_| EncryptionMethod::AES128),
                                    map(tag("SAMPLE-AES"), |_| EncryptionMethod::SampleAES),
                                )),
                            ),
                            |(_, method)| ExtXKeyAttributes::Method(method),
                        ),
                        map(
                            separated_pair(
                                tag("URI"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, uri)| ExtXKeyAttributes::Uri(uri.to_string()),
                        ),
                        map_res(
                            separated_pair(tag("IV"), char('='), hexadecimal),
                            |(_, iv)| {
                                u128::from_str_radix(iv, 16).map(|iv| ExtXKeyAttributes::Iv(iv))
                            },
                        ),
                        map(
                            separated_pair(
                                tag("KEYFORMAT"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, key_format)| ExtXKeyAttributes::KeyFormat(key_format.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("KEYFORMATVERSIONS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, key_format_versions)| {
                                ExtXKeyAttributes::KeyFormatVersions(
                                    key_format_versions.to_string(),
                                )
                            },
                        ),
                        map(not_line_ending_or_comma, |_| ExtXKeyAttributes::Unknown),
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
                        ExtXKeyAttributes::Method(v) => {
                            if method.is_some() {
                                return Err("");
                            }
                            method = Some(v);
                        }
                        ExtXKeyAttributes::Uri(v) => {
                            if uri.is_some() {
                                return Err("");
                            }
                            uri = Some(v);
                        }
                        ExtXKeyAttributes::Iv(v) => {
                            if iv.is_some() {
                                return Err("");
                            }
                            iv = Some(v);
                        }
                        ExtXKeyAttributes::KeyFormat(v) => {
                            if key_format.is_some() {
                                return Err("");
                            }
                            key_format = Some(v);
                        }
                        ExtXKeyAttributes::KeyFormatVersions(v) => {
                            if key_format_versions.is_some() {
                                return Err("");
                            }
                            key_format_versions = Some(v);
                        }
                        ExtXKeyAttributes::Unknown => {}
                    }
                }

                Ok(Tag::MediaSegment(MediaSegmentTag::ExtXKey(Key {
                    method: method.ok_or("")?,
                    uri: uri.ok_or("")?,
                    iv,
                    key_format,
                    key_format_versions,
                })))
            },
        )
        .parse(s)
    }
}
