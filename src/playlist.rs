use std::str::FromStr;

use nom::{
    Parser,
    branch::alt,
    character::complete::{char, line_ending, not_line_ending},
    combinator::map,
    error::Error as NomError,
    multi::separated_list0,
    sequence::preceded,
};

use crate::Error;

#[derive(Debug)]
pub struct Playlist {
    pub entries: Vec<String>,
    pub comments: Vec<String>,
}

enum Line {
    Comment(String),
    Uri(String),
    Empty,
}

impl FromStr for Playlist {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = separated_list0(
            line_ending::<_, NomError<&str>>,
            alt((
                preceded(
                    char('#'),
                    map(not_line_ending, |comment: &str| {
                        Line::Comment(comment.to_string())
                    }),
                ),
                map(not_line_ending, |s: &str| {
                    if s.is_empty() {
                        Line::Empty
                    } else {
                        Line::Uri(s.to_string())
                    }
                }),
            )),
        )
        .parse(s)
        .unwrap()
        .1
        .into_iter();

        let mut entries = Vec::new();
        let mut comments = Vec::new();

        for line in lines {
            match line {
                Line::Comment(comment) => {
                    comments.push(comment);
                }
                Line::Uri(uri) => {
                    entries.push(uri);
                }
                Line::Empty => {}
            }
        }

        Ok(Self { entries, comments })
    }
}
