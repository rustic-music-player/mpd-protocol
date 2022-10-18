use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

use crate::requests::Command;

use super::*;

pub fn parse_music_database(input: &str) -> IResult<&str, Command> {
    alt((
        basic_command("listplaylists", Command::ListPlaylists),
        lsinfo,
        load,
        list,
        find,
        list_playlist_info,
    ))(input)
}

fn lsinfo(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("lsinfo"),
            string_arg,
        ),
        |version| Command::ListInfo(version),
    )(input)
}

fn load(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("load"),
            string_arg,
        ),
        |version| Command::LoadPlaylist(version),
    )(input)
}

fn list(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("list"),
            string_arg,
        ),
        |r#type| Command::List(r#type),
    )(input)
}

fn find(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("find"),
            string_arg,
        ),
        |filter| Command::Find(filter),
    )(input)
}

fn list_playlist_info(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("listplaylistinfo"),
            string_arg,
        ),
        |playlist| Command::ListPlaylistInfo(playlist),
    )(input)
}
