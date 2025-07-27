use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::char,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};

use crate::{
    tags::{Tag, Tags, master_playlist::MasterPlaylistTags},
    version::Version,
};

#[derive(Debug)]
pub(crate) struct ExtXSessionData {
    pub(crate) data_id: String,
    pub(crate) entry: ExtXSessionDataEntry,
    pub(crate) language: Option<String>,
}

#[derive(Debug)]
pub(crate) enum ExtXSessionDataEntry {
    Value(String),
    Uri(String),
}

enum ExtXSessionDataAttributes {
    DataId(String),
    Value(String),
    Uri(String),
    Language(String),
    Unknown,
}

impl Tag for ExtXSessionData {
    const TAG_PREFIX: &'static str = "EXT-X-SESSION-DATA";

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
                                tag("DATA-ID"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, data_id)| ExtXSessionDataAttributes::DataId(data_id.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("VALUE"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, value)| ExtXSessionDataAttributes::Value(value.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("URI"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, uri)| ExtXSessionDataAttributes::Uri(uri.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("LANGUAGE"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, language)| {
                                ExtXSessionDataAttributes::Language(language.to_string())
                            },
                        ),
                        map(not_line_ending_or_comma, |_| {
                            ExtXSessionDataAttributes::Unknown
                        }),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut data_id = None;
                let mut value = None;
                let mut uri = None;
                let mut language = None;

                for attribute in attributes {
                    match attribute {
                        ExtXSessionDataAttributes::DataId(v) => {
                            if data_id.is_some() {
                                return Err("");
                            }
                            data_id = Some(v);
                        }
                        ExtXSessionDataAttributes::Value(v) => {
                            if value.is_some() {
                                return Err("");
                            }
                            value = Some(v);
                        }
                        ExtXSessionDataAttributes::Uri(v) => {
                            if uri.is_some() {
                                return Err("");
                            }
                            uri = Some(v);
                        }
                        ExtXSessionDataAttributes::Language(v) => {
                            if language.is_some() {
                                return Err("");
                            }
                            language = Some(v);
                        }

                        ExtXSessionDataAttributes::Unknown => {}
                    }
                }

                let entry = match (value, uri) {
                    (Some(_), Some(_)) => {
                        return Err("");
                    }
                    (Some(value), None) => ExtXSessionDataEntry::Value(value),
                    (None, Some(uri)) => ExtXSessionDataEntry::Uri(uri),
                    (None, None) => {
                        return Err("");
                    }
                };

                Ok(Tags::MasterPlaylist(MasterPlaylistTags::ExtXSessionData(
                    Self {
                        data_id: data_id.ok_or("")?,
                        entry,
                        language,
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
