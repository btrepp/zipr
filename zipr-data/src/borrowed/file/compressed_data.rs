use crate::CompressionMethod;

/// Data structure which represents compressed data
/// Note: This is will always point to the same 'chunk'
/// Of memory, so clone/copy won't duplicate that
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CompressedData<'a> {
    bytes: &'a [u8],
    crc32: u32,
    uncompressed_size: u32,
    compression_method: CompressionMethod,
}

impl<'a> CompressedData<'a> {
    pub fn create_unchecked(
        uncompressed_size: u32,
        compression_method: CompressionMethod,
        crc32: u32,
        bytes: &'a [u8],
    ) -> Self {
        CompressedData {
            uncompressed_size,
            compression_method,
            crc32,
            bytes,
        }
    }

    pub fn compression_method(&self) -> CompressionMethod {
        self.compression_method
    }
    pub fn crc32(&self) -> u32 {
        self.crc32
    }

    pub fn uncompressed_size(&self) -> u32 {
        self.uncompressed_size
    }

    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
}
