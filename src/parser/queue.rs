use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

use crate::requests::Command;

use super::*;

pub fn parse_queue(input: &str) -> IResult<&str, Command> {
    alt((
        basic_command("clear", Command::Clear),
        basic_command("playlistinfo", Command::PlaylistInfo),
        playlist_changes,
        add_id,
    ))(input)
}

fn playlist_changes(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("plchanges"),
            string_arg,
        ),
        Command::PlaylistChanges,
    )(input)
}

fn add_id(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("addid"),
            string_arg,
        ),
        Command::AddId,
    )(input)
}
