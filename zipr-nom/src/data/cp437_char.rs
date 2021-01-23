use nom::{
    combinator::{eof, map, rest},
    IResult,
};
use zipr_data::borrowed::OEM437Str;

/// Parses the entire input as a asciichar
pub fn parse_cp437_chars<'a>(input: &'a [u8]) -> IResult<&[u8], OEM437Str<'a>> {
    let (rem, chars) = map(rest, OEM437Str::from)(input)?;
    let (rem, _) = eof(rem)?;
    Ok((rem, chars))
}
