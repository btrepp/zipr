use nom::{
    combinator::{eof, map, rest},
    IResult,
};
use zipr_data::CP437Str;

/// Parses the entire input as a asciichar
pub fn parse_cp437_chars<'a>(input: &'a [u8]) -> IResult<&[u8], CP437Str<'a>> {
    let (rem, chars) = map(rest, CP437Str::from_slice)(input)?;
    let (rem, _) = eof(rem)?;
    Ok((rem, chars))
}
