use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alphanumeric1, char, digit1, one_of};
use nom::combinator::{map, map_opt, recognize, eof, peek};
use nom::IResult;
use nom::sequence::{delimited, pair};

pub fn bool_arg(input: &str) -> IResult<&str, bool> {
    map(
        one_of("01"),
        |c: char| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!()
        }
    )(input)
}

pub fn u32_arg(input: &str) -> IResult<&str, u32> {
    map_opt(
        digit1,
        |digits: &str| u32::from_str_radix(digits, 10).ok()
    )(input)
}

pub fn i32_arg(input: &str) -> IResult<&str, i32> {
    map_opt(
        alt((
            digit1,
            recognize(pair(char('-'), digit1))
        )),
        |digits: &str| i32::from_str_radix(digits, 10).ok()
    )(input)
}

pub fn string_arg(input: &str) -> IResult<&str, String> {
    map(
        alt((
            delimited(tag(" \""), take_until("\""), char('"')),
            delimited(char(' '), alphanumeric1, alt((
                tag(" "),
                eof,
                peek(tag("\n")),
            )))
        )),
        |string: &str| string.to_string()
    )(input)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::string_arg;

    #[test_case(" \"artist\"", "artist"; "artist quoted")]
    #[test_case(" artist", "artist"; "artist unquoted")]
    #[test_case(" album", "album"; "album unqouted")]
    fn string_arg_should_parse_argument(input: &str, expected: &str) {
        let expected = expected.to_string();

        let result = string_arg(input);

        assert_eq!(Ok(("", expected)), result);
    }
}
