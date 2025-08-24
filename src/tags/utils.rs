use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, hex_digit1, u8, u16},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair},
};

use crate::types::{
    DateTime, DateTimeDate, DateTimeTime, DateTimeTimezone, DateTimeTimezoneHourOffset,
};

pub(crate) fn date_time(s: &str) -> IResult<&str, DateTime> {
    map(
        (
            map_res(u16, |year| {
                if year >= 1000 && year <= 9999 {
                    Ok(year)
                } else {
                    Err("")
                }
            }),
            char('-'),
            map_res(u8, |month| {
                if month >= 1 && month <= 12 {
                    Ok(month)
                } else {
                    Err("")
                }
            }),
            char('-'),
            map_res(u8, |day| {
                if day >= 1 && day <= 31 {
                    Ok(day)
                } else {
                    Err("")
                }
            }),
            char('T'),
            map_res(u8, |hour| if hour < 24 { Ok(hour) } else { Err("") }),
            char(':'),
            map_res(u8, |minute| if minute < 60 { Ok(minute) } else { Err("") }),
            char(':'),
            map_res(u8, |second| if second <= 60 { Ok(second) } else { Err("") }),
            char('.'),
            map_res(u16, |millisecond| {
                if millisecond <= 999 {
                    Ok(millisecond)
                } else {
                    Err("")
                }
            }),
            alt((
                map(char('Z'), |_| None),
                preceded(
                    char('+'),
                    map(
                        separated_pair(
                            map_res(u8, |hour| if hour < 24 { Ok(hour) } else { Err("") }),
                            char(':'),
                            map_res(u8, |minute| if minute < 60 { Ok(minute) } else { Err("") }),
                        ),
                        |(hour, minute)| {
                            Some(DateTimeTimezone {
                                hour_offset: DateTimeTimezoneHourOffset::East(hour),
                                minute_offset: minute,
                            })
                        },
                    ),
                ),
                preceded(
                    char('-'),
                    map(
                        separated_pair(
                            map_res(u8, |hour| if hour < 24 { Ok(hour) } else { Err("") }),
                            char(':'),
                            map_res(u8, |minute| if minute < 60 { Ok(minute) } else { Err("") }),
                        ),
                        |(hour, minute)| {
                            Some(DateTimeTimezone {
                                hour_offset: DateTimeTimezoneHourOffset::West(hour),
                                minute_offset: minute,
                            })
                        },
                    ),
                ),
            )),
        ),
        |(year, _, month, _, day, _, hour, _, minute, _, second, _, millisecond, time_zone)| {
            DateTime {
                date: DateTimeDate { year, month, day },
                time: DateTimeTime {
                    hour,
                    minute,
                    second,
                    millisecond,
                },
                time_zone,
            }
        },
    )
    .parse(s)
}

pub(crate) fn hexadecimal(s: &str) -> IResult<&str, &str> {
    preceded(alt((tag("0x"), tag("0X"))), hex_digit1).parse(s)
}

pub(crate) fn not_line_ending_or_comma(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '\n' || c == '\r' || c == ',').parse(s)
}

pub(crate) fn not_quote(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '"').parse(s)
}

pub(crate) fn not_equal(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == '=').parse(s)
}
