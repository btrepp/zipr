use nom::{
    bytes::complete::take,
    combinator::{map_parser, map_res},
    IResult,
};
use zipr_data::borrowed::{OEM437Str, ZipPath, ZipPathError};

use super::cp437_char::parse_cp437_chars;

pub fn parse_zip_path<'a>(input: &'a [u8]) -> IResult<&[u8], ZipPath<'a>> {
    let to_path = |input: OEM437Str<'a>| -> Result<ZipPath<'a>, ZipPathError> {
        let result = ZipPath::from_cp437(input)?;
        Ok(result)
    };

    let len = input.len();
    let asciistr = map_parser(take(len), parse_cp437_chars);
    let mut parser = map_res(asciistr, to_path);
    let (input, file_name) = parser(input)?;
    Ok((input, file_name))
}
