use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;
use crate::ReplayGainMode;

use crate::requests::Command;

use super::*;

pub fn parse_playback_options(input: &str) -> IResult<&str, Command> {
    alt((
        random,
        repeat,
        set_volume,
        basic_command("getvol", Command::GetVolume),
        change_volume,
        basic_command("replay_gain_status", Command::ReplayGainStatus),
        replay_gain_mode,
    ))(input)
}

fn set_volume(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("setvol"),
            preceded(multispace0, u32_arg),
        ),
        |volume| Command::SetVolume(volume),
    )(input)
}

fn change_volume(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("volume"),
            preceded(multispace0, i32_arg),
        ),
        |volume| Command::ChangeVolumeBy(volume),
    )(input)
}

fn random(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("random"),
            preceded(multispace0, bool_arg),
        ),
        |random| Command::Random(random),
    )(input)
}

fn repeat(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("repeat"),
            preceded(multispace0, bool_arg),
        ),
        |repeat| Command::Repeat(repeat),
    )(input)
}

fn replay_gain_mode(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("replay_gain_mode"),
            preceded(multispace0, alt((
                enum_variant("off", ReplayGainMode::Off),
                enum_variant("track", ReplayGainMode::Track),
                enum_variant("album", ReplayGainMode::Album),
                enum_variant("auto", ReplayGainMode::Auto),
            ))),
        ),
        |mode| Command::ReplayGainMode(mode),
    )(input)
}

fn enum_variant<T: Copy + 'static>(arg: &str, variant: T) -> impl FnMut(&str) -> IResult<&str, T> + '_  {
    move |input: &str| map(
        tag(arg),
        |_| variant
    )(input)
}
