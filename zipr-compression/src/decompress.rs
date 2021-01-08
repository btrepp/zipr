use alloc::vec::Vec;
use miniz_oxide::inflate::TINFLStatus;
use zipr_core::data::CompressedData;

#[derive(Debug)]
pub enum DecompressError {
    InvalidCrc,
    UnhandledCompressionMethod,
    Miniz(TINFLStatus),
}
pub trait DecompressToVec {
    fn decompress_to_vec(&self) -> Result<Vec<u8>, DecompressError>;
}

impl DecompressToVec for CompressedData<'_> {
    fn decompress_to_vec(&self) -> Result<Vec<u8>, DecompressError> {
        let method = self.compression_method();
        match method {
            zipr_core::data::CompressionMethod::Stored => Ok(Vec::from(self.bytes())),
            zipr_core::data::CompressionMethod::Deflate => {
                let vec = miniz_oxide::inflate::decompress_to_vec(self.bytes());
                match vec {
                    Ok(x) => Ok(x),
                    Err(tinfl) => Err(DecompressError::Miniz(tinfl)),
                }
            }
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
        let crc32 = 0;
        let compresseddata = CompressedData::create_unchecked(
            uncompressed_size,
            zipr_core::data::CompressionMethod::Deflate,
            crc32,
            data,
        );

        let result = compresseddata.decompress_to_vec().unwrap();

        assert_eq!(repeated_times * 5, result.len());
        assert_eq!(expected, result);
    }
}
