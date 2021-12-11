use crc::crc32;
use miniz_oxide::deflate::compress_to_vec;
use zipr_data::{borrowed::file::CompressedData, CompressionMethod};

/// Compresses the data using deflate. Note: requires a buffer to store the newly deflated data in
pub fn deflate<'a>(output: &'a mut alloc::vec::Vec<u8>, bytes: &'_ [u8]) -> CompressedData<'a> {
    let uncompressed_size = bytes.len() as u32;
    let crc32 = crc32::checksum_ieee(bytes);
    let buffer = compress_to_vec(bytes, 1);
    output.resize(buffer.len(), 0);
    output.copy_from_slice(&buffer);
    let bytes = (*output).as_slice();
    CompressedData::create_unchecked(uncompressed_size, CompressionMethod::Deflate, crc32, bytes)
}

/// Just stores the data. Note: we do copy the data into the buffer, so we aren't dependant on the lifetime of
/// bytes
pub fn store<'a>(output: &'a mut alloc::vec::Vec<u8>, bytes: &'_ [u8]) -> CompressedData<'a> {
    let uncompressed_size = bytes.len() as u32;
    let crc32 = crc32::checksum_ieee(bytes);
    output.resize(bytes.len(), 0);
    output.copy_from_slice(bytes);
    let bytes = (*output).as_slice();
    CompressedData::create_unchecked(uncompressed_size, CompressionMethod::Stored, crc32, bytes)
}

/// Creates the compressed data from the supplied method and stores it in the buffer
/// A compressed data handle will be returned that can be used to zip
pub fn compress_with<'a>(
    compression: CompressionMethod,
    buffer: &'a mut alloc::vec::Vec<u8>,
    bytes: &'_ [u8],
) -> CompressedData<'a> {
    match compression {
        CompressionMethod::Deflate => deflate(buffer, bytes),
        CompressionMethod::Stored => store(buffer, bytes),
    }
}

#[cfg(test)]
mod tests {}
