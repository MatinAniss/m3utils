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
pub(crate) struct ExtXMedia {
    pub(crate) media_type: ExtXMediaMediaType,
    pub(crate) uri: Option<String>,
    pub(crate) group_id: String,
    pub(crate) language: Option<String>,
    pub(crate) associated_language: Option<String>,
    pub(crate) name: String,
    pub(crate) default: bool,
    pub(crate) autoselect: bool,
    pub(crate) forced: bool,
    pub(crate) instream_id: Option<String>,
    pub(crate) characteristics: Option<String>,
    pub(crate) channels: Option<String>,
}

#[derive(Debug)]
pub(crate) enum ExtXMediaMediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

enum ExtXMediaAttributes {
    Type(ExtXMediaMediaType),
    Uri(String),
    GroupId(String),
    Language(String),
    AssociatedLanguage(String),
    Name(String),
    Default(bool),
    Autoselect(bool),
    Forced(bool),
    InstreamId(String),
    Characteristics(String),
    Channels(String),
    Unknown,
}

impl Tag for ExtXMedia {
    const TAG_PREFIX: &'static str = "EXT-X-MEDIA";

    fn min_version() -> Version {
        Version::V1
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
                                tag("TYPE"),
                                char('='),
                                alt((
                                    map(tag("AUDIO"), |_| ExtXMediaMediaType::Audio),
                                    map(tag("VIDEO"), |_| ExtXMediaMediaType::Video),
                                    map(tag("SUBTITLES"), |_| ExtXMediaMediaType::Subtitles),
                                    map(tag("CLOSED-CAPTIONS"), |_| {
                                        ExtXMediaMediaType::ClosedCaptions
                                    }),
                                )),
                            ),
                            |(_, media_type)| ExtXMediaAttributes::Type(media_type),
                        ),
                        map(
                            separated_pair(
                                tag("URI"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, uri)| ExtXMediaAttributes::Uri(uri.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("GROUP-ID"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, group_id)| ExtXMediaAttributes::GroupId(group_id.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("LANGUAGE"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, language)| ExtXMediaAttributes::Language(language.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("ASSOC-LANGUAGE"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, associated_language)| {
                                ExtXMediaAttributes::AssociatedLanguage(
                                    associated_language.to_string(),
                                )
                            },
                        ),
                        map(
                            separated_pair(
                                tag("NAME"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, name)| ExtXMediaAttributes::Name(name.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("DEFAULT"),
                                char('='),
                                alt((map(tag("YES"), |_| true), map(tag("NO"), |_| false))),
                            ),
                            |(_, default)| ExtXMediaAttributes::Default(default),
                        ),
                        map(
                            separated_pair(
                                tag("AUTOSELECT"),
                                char('='),
                                alt((map(tag("YES"), |_| true), map(tag("NO"), |_| false))),
                            ),
                            |(_, autoselect)| ExtXMediaAttributes::Autoselect(autoselect),
                        ),
                        map(
                            separated_pair(
                                tag("FORCED"),
                                char('='),
                                alt((map(tag("YES"), |_| true), map(tag("NO"), |_| false))),
                            ),
                            |(_, forced)| ExtXMediaAttributes::Forced(forced),
                        ),
                        map(
                            separated_pair(
                                tag("INSTREAM-ID"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, instream_id)| {
                                ExtXMediaAttributes::InstreamId(instream_id.to_string())
                            },
                        ),
                        map(
                            separated_pair(
                                tag("CHARACTERISTICS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, characteristics)| {
                                ExtXMediaAttributes::Characteristics(characteristics.to_string())
                            },
                        ),
                        map(
                            separated_pair(
                                tag("CHANNELS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, channels)| ExtXMediaAttributes::Channels(channels.to_string()),
                        ),
                        map(not_line_ending_or_comma, |_| ExtXMediaAttributes::Unknown),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut media_type = None;
                let mut uri = None;
                let mut group_id = None;
                let mut language = None;
                let mut associated_language = None;
                let mut name = None;
                let mut default = None;
                let mut autoselect = None;
                let mut forced = None;
                let mut instream_id = None;
                let mut characteristics = None;
                let mut channels = None;

                for attribute in attributes {
                    match attribute {
                        ExtXMediaAttributes::Type(v) => {
                            if media_type.is_some() {
                                return Err("");
                            }
                            media_type = Some(v);
                        }
                        ExtXMediaAttributes::Uri(v) => {
                            if uri.is_some() {
                                return Err("");
                            }
                            uri = Some(v);
                        }
                        ExtXMediaAttributes::GroupId(v) => {
                            if group_id.is_some() {
                                return Err("");
                            }
                            group_id = Some(v);
                        }
                        ExtXMediaAttributes::Language(v) => {
                            if language.is_some() {
                                return Err("");
                            }
                            language = Some(v);
                        }
                        ExtXMediaAttributes::AssociatedLanguage(v) => {
                            if associated_language.is_some() {
                                return Err("");
                            }
                            associated_language = Some(v);
                        }
                        ExtXMediaAttributes::Name(v) => {
                            if name.is_some() {
                                return Err("");
                            }
                            name = Some(v);
                        }
                        ExtXMediaAttributes::Default(v) => {
                            if default.is_some() {
                                return Err("");
                            }
                            default = Some(v);
                        }
                        ExtXMediaAttributes::Autoselect(v) => {
                            if autoselect.is_some() {
                                return Err("");
                            }
                            autoselect = Some(v);
                        }
                        ExtXMediaAttributes::Forced(v) => {
                            if forced.is_some() {
                                return Err("");
                            }
                            forced = Some(v);
                        }
                        ExtXMediaAttributes::InstreamId(v) => {
                            if instream_id.is_some() {
                                return Err("");
                            }
                            instream_id = Some(v);
                        }
                        ExtXMediaAttributes::Characteristics(v) => {
                            if characteristics.is_some() {
                                return Err("");
                            }
                            characteristics = Some(v);
                        }
                        ExtXMediaAttributes::Channels(v) => {
                            if channels.is_some() {
                                return Err("");
                            }
                            channels = Some(v);
                        }
                        ExtXMediaAttributes::Unknown => {}
                    }
                }

                Ok(Tags::MasterPlaylist(MasterPlaylistTags::ExtXMedia(Self {
                    media_type: media_type.ok_or("")?,
                    uri,
                    group_id: group_id.ok_or("")?,
                    language,
                    associated_language,
                    name: name.ok_or("")?,
                    default: default.unwrap_or(false),
                    autoselect: autoselect.unwrap_or(false),
                    forced: forced.unwrap_or(false),
                    instream_id,
                    characteristics,
                    channels,
                })))
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
