use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::IResult;
use nom::sequence::preceded;

use crate::requests::Command;

use super::*;

pub fn parse_controlling_playback(input: &str) -> IResult<&str, Command> {
    alt((
        basic_command("next", Command::Next),
        pause,
        play_id,
        play,
        basic_command("previous", Command::Previous),
        basic_command("stop", Command::Stop),
    ))(input)
}

fn pause(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("pause"),
            opt(preceded(multispace0, bool_arg)),
        ),
        |paused| Command::Pause(paused),
    )(input)
}

fn play(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("play"),
            opt(preceded(multispace0, u32_arg)),
        ),
        |song_pos| Command::Play(song_pos),
    )(input)
}

fn play_id(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("playid"),
            opt(preceded(multispace0, u32_arg)),
        ),
        |song_id| Command::PlayId(song_id),
    )(input)
}
