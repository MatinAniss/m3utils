use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_res},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
};

use crate::{
    tags::{
        ExtTag, MediaSegmentTag, Tag,
        utils::{date_time, hexadecimal, not_equal, not_line_ending_or_comma, not_quote},
    },
    types::{ClientAttributeValue, DateRange, DateTime, Version},
};

#[derive(Debug)]
pub struct ExtXDaterange {
    pub id: String,
    pub class: Option<String>,
    pub start_date: DateTime,
    pub end_date: Option<DateTime>,
    pub duration: Option<f64>,
    pub planned_duration: Option<f64>,
    pub client_attributes: Vec<(String, ClientAttributeValue)>,
    pub end_on_next: bool,
}

enum ExtXDaterangeAttributes {
    Id(String),
    Class(String),
    StartDate(DateTime),
    EndDate(DateTime),
    Duration(f64),
    PlannedDuration(f64),
    ClientAttribute((String, ClientAttributeValue)),
    EndOnNext,
    Unknown,
}

impl ExtTag for ExtXDaterange {
    const TAG_PREFIX: &'static str = "EXT-X-DATERANGE";

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
                                tag("ID"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, id)| ExtXDaterangeAttributes::Id(id.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("CLASS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, class)| ExtXDaterangeAttributes::Class(class.to_string()),
                        ),
                        map(
                            separated_pair(tag("START-DATE"), char('='), date_time),
                            |(_, date_time)| ExtXDaterangeAttributes::StartDate(date_time),
                        ),
                        map(
                            separated_pair(tag("END-DATE"), char('='), date_time),
                            |(_, date_time)| ExtXDaterangeAttributes::EndDate(date_time),
                        ),
                        map(
                            separated_pair(tag("DURATION"), char('='), double),
                            |(_, duration)| ExtXDaterangeAttributes::Duration(duration),
                        ),
                        map(
                            separated_pair(tag("PLANNED-DURATION"), char('='), double),
                            |(_, planned_duration)| {
                                ExtXDaterangeAttributes::PlannedDuration(planned_duration)
                            },
                        ),
                        map(
                            separated_pair(
                                map(preceded(tag("X-"), not_equal), |name| format!("X-{name}")),
                                char('='),
                                alt((
                                    map(delimited(char('"'), not_quote, char('"')), |string| {
                                        ClientAttributeValue::String(string.to_string())
                                    }),
                                    map(hexadecimal, |hexadecimal| {
                                        ClientAttributeValue::Hexadecimal(hexadecimal.to_string())
                                    }),
                                    map(double, |decimal| ClientAttributeValue::F64(decimal)),
                                )),
                            ),
                            |client_attribute| {
                                ExtXDaterangeAttributes::ClientAttribute(client_attribute)
                            },
                        ),
                        map(
                            separated_pair(tag("END-ON-NEXT"), char('='), tag("YES")),
                            |_| ExtXDaterangeAttributes::EndOnNext,
                        ),
                        map(not_line_ending_or_comma, |_| {
                            ExtXDaterangeAttributes::Unknown
                        }),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut id = None;
                let mut class = None;
                let mut start_date = None;
                let mut end_date = None;
                let mut duration = None;
                let mut planned_duration = None;
                let mut client_attributes = Vec::new();
                let mut end_on_next = None;

                for attribute in attributes {
                    match attribute {
                        ExtXDaterangeAttributes::Id(v) => {
                            if id.is_some() {
                                return Err("");
                            }
                            id = Some(v);
                        }
                        ExtXDaterangeAttributes::Class(v) => {
                            if class.is_some() {
                                return Err("");
                            }
                            class = Some(v);
                        }
                        ExtXDaterangeAttributes::StartDate(v) => {
                            if start_date.is_some() {
                                return Err("");
                            }
                            start_date = Some(v);
                        }
                        ExtXDaterangeAttributes::EndDate(v) => {
                            if end_date.is_some() {
                                return Err("");
                            }
                            end_date = Some(v);
                        }
                        ExtXDaterangeAttributes::Duration(v) => {
                            if duration.is_some() {
                                return Err("");
                            }
                            duration = Some(v);
                        }
                        ExtXDaterangeAttributes::PlannedDuration(v) => {
                            if planned_duration.is_some() {
                                return Err("");
                            }
                            planned_duration = Some(v);
                        }
                        ExtXDaterangeAttributes::ClientAttribute(v) => {
                            client_attributes.push(v);
                        }
                        ExtXDaterangeAttributes::EndOnNext => {
                            if end_on_next.is_some() {
                                return Err("");
                            }
                            end_on_next = Some(true);
                        }
                        ExtXDaterangeAttributes::Unknown => {}
                    }
                }

                Ok(Tag::MediaSegment(MediaSegmentTag::ExtXDaterange(
                    DateRange {
                        id: id.ok_or("")?,
                        class,
                        start_date: start_date.ok_or("")?,
                        end_date,
                        duration,
                        planned_duration,
                        client_attributes,
                        end_on_next: end_on_next.unwrap_or(false),
                    },
                )))
            },
        )
        .parse(s)
    }
}
