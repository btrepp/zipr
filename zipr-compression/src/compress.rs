use crc::crc32;
use zipr_core::data::{file::CompressedData, CompressionMethod};

pub fn deflate(bytes: &[u8]) -> CompressedData<'_> {
    let compression_method = CompressionMethod::Deflate;
    let uncompressed_size = bytes.len() as u32;
    let crc32 = crc32::checksum_ieee(bytes);
    CompressedData::create_unchecked(uncompressed_size, compression_method, crc32, bytes)
}

#[cfg(test)]
mod tests {}
