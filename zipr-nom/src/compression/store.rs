use nom::IResult;

/// Parses out the data if it's stored data.
/// Literally is the id function
/// Consumes all the data
pub fn parse_store<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    Ok((&[] as &[u8], input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_store() {
        let expected = "world".as_bytes();
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let entry_position = 0x0;
        let header_length = 39; // see domain tests for offset
        let data_start = entry_position + header_length;
        let data_end = data_start + expected.len();
        let data = &hello[data_start..data_end];
        let result = parse_store(data);

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
