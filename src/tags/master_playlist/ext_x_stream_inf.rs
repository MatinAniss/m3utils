use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::{map, map_res},
    multi::separated_list0,
    number::complete::double,
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
pub struct ExtXStreamInf {
    pub bandwidth: u64,
    pub average_bandwidth: Option<u64>,
    pub codecs: String,
    pub resolution: Option<Resolution>,
    pub frame_rate: Option<f64>,
    pub hdcp_level: Option<HDCPLevel>,
    pub audio: Option<String>,
    pub video: Option<String>,
    pub subtitles: Option<String>,
    pub closed_captions: Option<String>,
}

enum ExtXStreamInfAttributes {
    Bandwidth(u64),
    AverageBandwidth(u64),
    Codecs(String),
    Resolution(Resolution),
    FrameRate(f64),
    HDCPLevel(HDCPLevel),
    Audio(String),
    Video(String),
    Subtitles(String),
    ClosedCaptions(String),
    Unknown,
}

impl Tag for ExtXStreamInf {
    const TAG_PREFIX: &'static str = "EXT-X-STREAM-INF";

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
                            separated_pair(tag("BANDWIDTH"), char('='), u64),
                            |(_, bandwidth)| ExtXStreamInfAttributes::Bandwidth(bandwidth),
                        ),
                        map(
                            separated_pair(tag("AVERAGE-BANDWITH"), char('='), u64),
                            |(_, average_bandwidth)| {
                                ExtXStreamInfAttributes::AverageBandwidth(average_bandwidth)
                            },
                        ),
                        map(
                            separated_pair(
                                tag("CODECS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, codecs)| ExtXStreamInfAttributes::Codecs(codecs.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("RESOLUTION"),
                                char('='),
                                separated_pair(u64, char('x'), u64),
                            ),
                            |(_, (width, height))| {
                                ExtXStreamInfAttributes::Resolution(Resolution { width, height })
                            },
                        ),
                        map(
                            separated_pair(tag("FRAME-RATE"), char('='), double),
                            |(_, frame_rate)| ExtXStreamInfAttributes::FrameRate(frame_rate),
                        ),
                        map(
                            separated_pair(
                                tag("HDCP-LEVEL"),
                                char('='),
                                map(tag("TYPE-0"), |_| HDCPLevel::Type0),
                            ),
                            |(_, hdcp_level)| ExtXStreamInfAttributes::HDCPLevel(hdcp_level),
                        ),
                        map(
                            separated_pair(
                                tag("AUDIO"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, audio)| ExtXStreamInfAttributes::Audio(audio.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("VIDEO"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, video)| ExtXStreamInfAttributes::Video(video.to_string()),
                        ),
                        map(
                            separated_pair(
                                tag("SUBTITLES"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, subtitles)| {
                                ExtXStreamInfAttributes::Subtitles(subtitles.to_string())
                            },
                        ),
                        map(
                            separated_pair(
                                tag("CLOSED-CAPTIONS"),
                                char('='),
                                delimited(char('"'), not_quote, char('"')),
                            ),
                            |(_, closed_captions)| {
                                ExtXStreamInfAttributes::ClosedCaptions(closed_captions.to_string())
                            },
                        ),
                        map(not_line_ending_or_comma, |_| {
                            ExtXStreamInfAttributes::Unknown
                        }),
                    )),
                ),
            ),
            |(_, attributes)| {
                let mut bandwidth = None;
                let mut average_bandwidth = None;
                let mut codecs = None;
                let mut resolution = None;
                let mut frame_rate = None;
                let mut hdcp_level = None;
                let mut audio = None;
                let mut video = None;
                let mut subtitles = None;
                let mut closed_captions = None;

                for attribute in attributes {
                    match attribute {
                        ExtXStreamInfAttributes::Bandwidth(v) => {
                            if bandwidth.is_some() {
                                return Err("");
                            }
                            bandwidth = Some(v);
                        }
                        ExtXStreamInfAttributes::AverageBandwidth(v) => {
                            if average_bandwidth.is_some() {
                                return Err("");
                            }
                            average_bandwidth = Some(v);
                        }
                        ExtXStreamInfAttributes::Codecs(v) => {
                            if codecs.is_some() {
                                return Err("");
                            }
                            codecs = Some(v);
                        }
                        ExtXStreamInfAttributes::Resolution(v) => {
                            if resolution.is_some() {
                                return Err("");
                            }
                            resolution = Some(v);
                        }
                        ExtXStreamInfAttributes::FrameRate(v) => {
                            if frame_rate.is_some() {
                                return Err("");
                            }
                            frame_rate = Some(v);
                        }
                        ExtXStreamInfAttributes::HDCPLevel(v) => {
                            if hdcp_level.is_some() {
                                return Err("");
                            }
                            hdcp_level = Some(v);
                        }
                        ExtXStreamInfAttributes::Audio(v) => {
                            if audio.is_some() {
                                return Err("");
                            }
                            audio = Some(v);
                        }
                        ExtXStreamInfAttributes::Video(v) => {
                            if video.is_some() {
                                return Err("");
                            }
                            video = Some(v);
                        }
                        ExtXStreamInfAttributes::Subtitles(v) => {
                            if subtitles.is_some() {
                                return Err("");
                            }
                            subtitles = Some(v);
                        }
                        ExtXStreamInfAttributes::ClosedCaptions(v) => {
                            if closed_captions.is_some() {
                                return Err("");
                            }
                            closed_captions = Some(v);
                        }
                        ExtXStreamInfAttributes::Unknown => {}
                    }
                }

                Ok(Tags::MasterPlaylist(MasterPlaylistTags::ExtXStreamInf(
                    Self {
                        bandwidth: bandwidth.ok_or("")?,
                        average_bandwidth,
                        codecs: codecs.ok_or("")?,
                        resolution,
                        frame_rate,
                        hdcp_level,
                        video,
                        audio,
                        subtitles,
                        closed_captions,
                    },
                )))
            },
        )
        .parse(s)
    }
}
