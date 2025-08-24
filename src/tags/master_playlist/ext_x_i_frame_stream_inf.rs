use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};

use crate::{
    tags::{
        Tag, Tags,
        master_playlist::MasterPlaylistTags,
        utils::{not_line_ending_or_comma, not_quote},
    },
    types::{HDCPLevel, Resolution, Version},
};

#[derive(Debug)]
pub struct ExtXIFrameStreamInf {
    pub bandwidth: u64,
    pub average_bandwidth: Option<u64>,
    pub codecs: Option<String>,
    pub resolution: Option<Resolution>,
    pub hdcp_level: Option<HDCPLevel>,
    pub video: Option<String>,
    pub uri: String,
}

enum ExtXIFrameStreamInfAttributes {
    Bandwidth(u64),
    AverageBandwidth(u64),
    Codecs(String),
    Resolution(Resolution),
    HDCPLevel(HDCPLevel),
    Video(String),
    Uri(String),
    Unknown,
}

impl Tag for ExtXIFrameStreamInf {
    const TAG_PREFIX: &'static str = "EXT-X-I-FRAME-STREAM-INF";

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
                            separated_pair(tag("BANDWIDTH"), char('='), u64),
                            |(_, bandwidth)| ExtXIFrameStreamInfAttributes::Bandwidth(bandwidth),
                        ),
                        map(
                            separated_pair(tag("AVERAGE-BANDWITH"), char('='), u64),
                            |(_, average_bandwidth)| {
                                ExtXIFrameStreamInfAttributes::AverageBandwidth(average_bandwidth)
                            },
                        ),
                        map(
                            separated_pair(
                                tag("CODECS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, codecs)| ExtXIFrameStreamInfAttributes::Codecs(codecs.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("RESOLUTION"),
                                char('='),
                                separated_pair(u64, char('x'), u64),
                            ),
                            |(_, (width, height))| {
                                ExtXIFrameStreamInfAttributes::Resolution(Resolution {
                                    width,
                                    height,
                                })
                            },
                        ),
                        map(
                            separated_pair(
                                tag("HDCP-LEVEL"),
                                char('='),
                                map(tag("TYPE-0"), |_| HDCPLevel::Type0),
                            ),
                            |(_, hdcp_level)| ExtXIFrameStreamInfAttributes::HDCPLevel(hdcp_level),
                        ),
                        map(
                            separated_pair(
                                tag("VIDEO"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, video)| ExtXIFrameStreamInfAttributes::Video(video.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("URI"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, uri)| ExtXIFrameStreamInfAttributes::Uri(uri.to_string()),
                        ),
                        map(not_line_ending_or_comma, |_| {
                            ExtXIFrameStreamInfAttributes::Unknown
                        }),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut bandwidth = None;
                let mut average_bandwidth = None;
                let mut codecs = None;
                let mut resolution = None;
                let mut hdcp_level = None;
                let mut video = None;
                let mut uri = None;

                for attribute in attributes {
                    match attribute {
                        ExtXIFrameStreamInfAttributes::Bandwidth(v) => {
                            if bandwidth.is_some() {
                                return Err("");
                            }
                            bandwidth = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::AverageBandwidth(v) => {
                            if average_bandwidth.is_some() {
                                return Err("");
                            }
                            average_bandwidth = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::Codecs(v) => {
                            if codecs.is_some() {
                                return Err("");
                            }
                            codecs = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::Resolution(v) => {
                            if resolution.is_some() {
                                return Err("");
                            }
                            resolution = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::HDCPLevel(v) => {
                            if hdcp_level.is_some() {
                                return Err("");
                            }
                            hdcp_level = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::Video(v) => {
                            if video.is_some() {
                                return Err("");
                            }
                            video = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::Uri(v) => {
                            if uri.is_some() {
                                return Err("");
                            }
                            uri = Some(v);
                        }
                        ExtXIFrameStreamInfAttributes::Unknown => {}
                    }
                }

                Ok(Tags::MasterPlaylist(
                    MasterPlaylistTags::ExtXIFrameStreamInf(Self {
                        bandwidth: bandwidth.ok_or("")?,
                        average_bandwidth,
                        codecs,
                        resolution,
                        hdcp_level,
                        video,
                        uri: uri.ok_or("")?,
                    }),
                ))
            },
        )
        .parse(s)
    }
}
