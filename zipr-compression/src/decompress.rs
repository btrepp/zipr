use alloc::vec::Vec;
use crc::Crc;
use crc::CRC_32_ISO_HDLC;
use miniz_oxide::inflate::TINFLStatus;
use zipr_data::{borrowed::file::CompressedData, CompressionMethod};

const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

/// Enum for ways in which decompression can fail
#[derive(Debug)]
pub enum DecompressError {
    InvalidCrc(u32, u32),
    UnhandledCompressionMethod,
    Miniz(TINFLStatus),
}

/// Trait for expanded the compressed data into a vector
pub trait DecompressToVec {
    fn decompress_to_vec(&self) -> Result<Vec<u8>, DecompressError>;
}

impl DecompressToVec for CompressedData<'_> {
    fn decompress_to_vec(&self) -> Result<Vec<u8>, DecompressError> {
        let method = self.compression_method();
        let bytes = match method {
            CompressionMethod::Stored => Ok(Vec::from(self.bytes())),
            CompressionMethod::Deflate => {
                let vec = miniz_oxide::inflate::decompress_to_vec(self.bytes());
                match vec {
                    Ok(x) => Ok(x),
                    Err(tinfl) => Err(DecompressError::Miniz(tinfl)),
                }
            }
        }?;

        let crc = CRC32.checksum(&bytes);

        if crc == self.crc32() {
            Ok(bytes)
        } else {
            Err(DecompressError::InvalidCrc(self.crc32(), crc))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_hello_world_deflate() {
        let repeated_times = 43;
        let expected = "world".repeat(repeated_times).as_bytes().to_vec();
        let hello = include_bytes!("../../assets/hello_world_deflate.zip");
        let entry_position = 0x0;
        let header_length = 39; // see domain tests for offset
        let deflated_size = 22;
        let data_start = entry_position + header_length;
        let data_end = data_start + deflated_size;
        let data = &hello[data_start..data_end];
        let uncompressed_size = 100;
        let crc32 = 810231625;
        let compresseddata = CompressedData::create_unchecked(
            uncompressed_size,
            CompressionMethod::Deflate,
            crc32,
            data,
        );

        let result = compresseddata.decompress_to_vec().unwrap();

        assert_eq!(repeated_times * 5, result.len());
        assert_eq!(expected, result);
    }
}
