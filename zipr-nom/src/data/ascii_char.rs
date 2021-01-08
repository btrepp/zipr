use ascii::{AsAsciiStrError, AsciiStr};
use nom::{
    combinator::{eof, map_res, rest},
    IResult,
};

fn convert_chars(input: &[u8]) -> Result<&AsciiStr, AsAsciiStrError> {
    let res = AsciiStr::from_ascii(input)?;
    Ok(res)
}

/// Parses the entire input as a asciichar
pub fn parse_ascii_chars(input: &[u8]) -> IResult<&[u8], &AsciiStr> {
    let (rem, chars) = map_res(rest, convert_chars)(input)?;
    let (rem, _) = eof(rem)?;
    Ok((rem, chars))
}
