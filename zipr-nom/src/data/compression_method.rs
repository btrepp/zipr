use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use zipr_core::{
    constants::{COMPRESSION_DEFLATE, COMPRESSION_STORED},
    data::CompressionMethod,
};

/// Uses alternatives to try and pass the extra field.
/// Will return one of the datatypes, or fail
pub fn parse_compression_method(input: &[u8]) -> IResult<&[u8], CompressionMethod> {
    let (input, method) = alt((
        map(tag(COMPRESSION_STORED), |_| CompressionMethod::Stored),
        map(tag(COMPRESSION_DEFLATE), |_| CompressionMethod::Deflate),
    ))(input)?;

    Ok((input, method))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_stored() {
        let input: &[u8] = &[0x00, 0x00];
        let (rem, result) = parse_compression_method(input).unwrap();
        assert_eq!(&[] as &[u8], rem);
        assert_eq!(CompressionMethod::Stored, result);
    }

    #[test]
    fn compression_deflate() {
        let input: &[u8] = &[0x08, 0x00];
        let (rem, result) = parse_compression_method(input).unwrap();
        assert_eq!(&[] as &[u8], rem);
        assert_eq!(CompressionMethod::Deflate, result);
    }
}
