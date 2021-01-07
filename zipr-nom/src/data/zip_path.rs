use nom::{bytes::complete::take, combinator::map_res, IResult};
use zipr_core::data::{ZipPath, ZipPathError};

pub fn parse_zip_path<'a>(input: &'a [u8]) -> IResult<&[u8], ZipPath<'a>> {
    let to_path = |bytes: &'a [u8]| -> Result<ZipPath<'a>, ZipPathError> {
        let result = ZipPath::create_from_bytes(bytes)?;
        Ok(result)
    };
    let len = input.len();
    let (input, file_name) = map_res(take(len), to_path)(input)?;
    Ok((input, file_name))
}
