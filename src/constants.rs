pub const END_OF_CENTRAL_DIRECTORY_HEADER: [u8; 4] = [0x50, 0x4B, 0x05, 0x06];
pub const END_OF_CENTRAL_DIRECTORY_MIN_SIZE: usize = 22;
pub const CENTRAL_DIRECTORY_HEADER: [u8; 4] = [0x50, 0x4B, 0x01, 0x02];

pub const EXTRA_FIELD_NTFS_HEADER: [u8; 2] = [0x0a, 0x00];

pub const COMPRESSION_STORED: [u8; 2] = [0x00, 0x00];
pub const COMPRESSION_DEFLATE: [u8; 2] = [0x08, 0x00];
