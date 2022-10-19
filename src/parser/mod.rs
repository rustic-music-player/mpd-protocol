use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::{char};
use nom::combinator::{map};
use nom::IResult;
use nom::multi::{ separated_list0};
use nom::sequence::{delimited,  terminated};
use crate::parser::partition::parse_partitions;
use crate::parser::reflection::parse_reflection;
use crate::requests::{Command, Request};
pub(crate) use self::args::*;
use self::status::parse_mpd_status;
use self::controlling_playback::parse_controlling_playback;
use self::queue::parse_queue;
use self::music_database::parse_music_database;
use self::playback_options::parse_playback_options;

mod args;
mod status;
mod playback_options;
mod controlling_playback;
mod queue;
mod music_database;
mod partition;
mod reflection;

pub fn parse(input: &str) -> IResult<&str, Request> {
    alt((
        command,
        command_ok_list,
        command_list,
    ))(input)
}

fn command_ok_list(input: &str) -> IResult<&str, Request> {
    map(delimited(
        tag("command_list_ok_begin\n"),
        separated_list0(char('\n'), parse_command),
        tag("command_list_end\n")
    ), |commands: Vec<Command>| Request::CommandList(commands, true))(input)
}

fn command_list(input: &str) -> IResult<&str, Request> {
    map(delimited(
        tag("command_list_begin\n"),
        separated_list0(char('\n'), parse_command),
        tag("command_list_end\n")
    ), |commands: Vec<Command>| Request::CommandList(commands, false))(input)
}

fn command(input: &str) -> IResult<&str, Request> {
    map(terminated(parse_command, char('\n')), |cmd| Request::Command(cmd))(input)
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        parse_mpd_status,
        parse_music_database,
        parse_playback_options,
        parse_queue,
        parse_controlling_playback,
        parse_partitions,
        parse_reflection,
        basic_command("noidle", Command::NoIdle),
        basic_command("close", Command::Close),
        basic_command("tagtypes", Command::TagTypes),
        basic_command("outputs", Command::Outputs),
    ))(input)
}

pub(crate) fn basic_command(identifier: &str, command: Command) -> impl FnMut(&str) -> IResult<&str, Command> + '_ {
    move |input| map(
        tag(identifier),
        |_| command.clone(),
    )(input)
}
