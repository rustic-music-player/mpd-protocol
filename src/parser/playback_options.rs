use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

use crate::requests::Command;

use super::*;

pub fn parse_playback_options(input: &str) -> IResult<&str, Command> {
    alt((
        random,
        repeat,
        set_volume,
        basic_command("getvol", Command::GetVolume),
        change_volume,
    ))(input)
}

fn set_volume(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("setvol "),
            u32_arg,
        ),
        |volume| Command::SetVolume(volume),
    )(input)
}

fn change_volume(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("volume "),
            i32_arg,
        ),
        |volume| Command::ChangeVolumeBy(volume),
    )(input)
}

fn random(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("random "),
            bool_arg,
        ),
        |random| Command::Random(random),
    )(input)
}

fn repeat(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("repeat "),
            bool_arg,
        ),
        |repeat| Command::Repeat(repeat),
    )(input)
}
