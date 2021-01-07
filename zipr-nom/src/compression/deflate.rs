use flate2::bufread::DeflateDecoder;
use nom::{bytes::complete::take, combinator::map_res, IResult};
use std::io::prelude::*;

/// Parses out the data if it's deflat
/// uses flate2 internaly
pub fn parse_deflate<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<u8>> {
    let decode = |bytes: &'a [u8]| -> Result<Vec<u8>, std::io::Error> {
        let mut decoder = DeflateDecoder::new(bytes);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf)?;
        Ok(buf)
    };
    let (input, result) = map_res(take(input.len()), decode)(input)?;

    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_hello_world_deflate() {
        let repeated_times = 43;
        let expected = "world".repeat(repeated_times).as_bytes().to_vec();
        let hello = include_bytes!("../../../assets/hello_world_deflate.zip");
        let entry_position = 0x0;
        let header_length = 39; // see domain tests for offset
        let deflated_size = 22;
        let data_start = entry_position + header_length;
        let data_end = data_start + deflated_size;
        let data = &hello[data_start..data_end];
        let result = parse_deflate(data);

        let (rem, result) = result.unwrap();
        assert_eq!(0, rem.len());
        assert_eq!(repeated_times * 5, result.len());
        assert_eq!(expected, result);
    }
}
