use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::preceded;

use crate::requests::Command;

use super::*;

pub fn parse_mpd_status(input: &str) -> IResult<&str, Command> {
    alt((
        idle,
        basic_command("currentsong", Command::CurrentSong),
        basic_command("status", Command::Status),
        basic_command("stats", Command::Stats),
    ))(input)
}

fn idle(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("idle"),
            many0(string_arg),
        ),
        |systems: Vec<String>| Command::Idle(systems),
    )(input)
}
