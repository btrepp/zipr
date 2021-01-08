mod deflate;
mod store;

pub use deflate::parse_deflate;
use nom::{combinator::into, error::Error, IResult};
pub use store::parse_store;
use zipr_core::data::{CompressedData, CompressionMethod};
use alloc::vec::Vec;

pub fn parse_compressed_data<'a>(
    input: &'a CompressedData<'a>,
) -> IResult<&'a CompressedData<'a>, Vec<u8>> {
    let result = match input.compression_method() {
        CompressionMethod::Deflate => parse_deflate(input.bytes()),
        CompressionMethod::Stored => into(parse_store)(input.bytes()),
    }
    .map(|(_, vec)| (input, vec))
    .map_err(|e| e.map(|x| Error::new(input, x.code)));
    result
}
