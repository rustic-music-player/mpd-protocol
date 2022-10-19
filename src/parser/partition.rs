use nom::IResult;

use crate::requests::Command;

use super::*;

pub fn parse_partitions(input: &str) -> IResult<&str, Command> {
    basic_command("listpartitions", Command::ListPartitions)(input)
}
