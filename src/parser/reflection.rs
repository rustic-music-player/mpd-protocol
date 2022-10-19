use nom::IResult;

use crate::requests::Command;

use super::*;

pub fn parse_reflection(input: &str) -> IResult<&str, Command> {
    alt((
        basic_command("urlhandlers", Command::UrlHandlers),
        basic_command("commands", Command::Commands),
        basic_command("decoders", Command::Decoders),
    ))(input)
}
