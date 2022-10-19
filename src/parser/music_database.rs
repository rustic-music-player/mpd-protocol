use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::IResult;
use nom::sequence::{pair, preceded};

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
        albumart,
    ))(input)
}

fn lsinfo(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("lsinfo"),
            opt(string_arg),
        ),
        Command::ListInfo,
    )(input)
}

fn load(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("load"),
            string_arg,
        ),
        Command::LoadPlaylist,
    )(input)
}

fn list(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("list"),
            string_arg,
        ),
        Command::List,
    )(input)
}

fn find(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("find"),
            string_arg,
        ),
        Command::Find,
    )(input)
}

fn list_playlist_info(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("listplaylistinfo"),
            string_arg,
        ),
        Command::ListPlaylistInfo,
    )(input)
}

fn albumart(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("albumart"),
            pair(string_arg, preceded(multispace0, u32_arg)),
        ),
        |(uri, offset)| Command::AlbumArt(uri, offset),
    )(input)
}
