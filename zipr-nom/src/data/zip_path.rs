use ascii::AsciiStr;
use nom::{
    bytes::complete::take,
    combinator::{map_parser, map_res},
    IResult,
};
use zipr_core::data::{ZipPath, ZipPathError};

use super::ascii_char::parse_ascii_chars;

pub fn parse_zip_path<'a>(input: &'a [u8]) -> IResult<&[u8], ZipPath<'a>> {
    let to_path = |input: &'a AsciiStr| -> Result<ZipPath<'a>, ZipPathError> {
        let result = ZipPath::create_from_string(input)?;
        Ok(result)
    };

    let len = input.len();
    let asciistr = map_parser(take(len), parse_ascii_chars);
    let mut parser = map_res(asciistr, to_path);
    let (input, file_name) = parser(input)?;
    Ok((input, file_name))
}
