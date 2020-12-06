use std::{path::Path, str::{Utf8Error, from_utf8}};

use nom::{IResult, combinator::map_res, bytes::complete::take};


pub fn parse_path<'a>(input: &'a [u8]) -> IResult<&[u8], &'a Path> {

    let to_path = | bytes: &'a [u8] | -> Result<&'a Path, Utf8Error> {
        let utf8 = from_utf8(bytes)?;
        Ok(Path::new(utf8))
    };
    let len = input.len();
    let (input, file_name) = map_res(take(len), to_path)(input)?;
    Ok((input, file_name))
}