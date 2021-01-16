use cookie_factory::{combinator::slice, lib::std::io::Write, SerializeFn};
use zipr_core::{
    constants::{COMPRESSION_DEFLATE, COMPRESSION_STORED},
    data::CompressionMethod,
};

pub fn compression_method<W: Write>(input: &CompressionMethod) -> impl SerializeFn<W> {
    let bytes = match input {
        CompressionMethod::Stored => COMPRESSION_STORED,
        CompressionMethod::Deflate => COMPRESSION_DEFLATE,
    };
    slice(bytes)
}

#[cfg(test)]
mod tests {
    use cookie_factory::gen;

    use super::*;

    #[test]
    fn compression_stored() {
        let expected: &[u8] = &[0x00, 0x00];
        let mut buf = [0u8; 2];
        let serializer = compression_method(&CompressionMethod::Stored);
        let (_, _) = gen(serializer, &mut buf[..]).unwrap();
        assert_eq!(expected, buf);
    }

    #[test]
    fn compression_delflate() {
        let expected: &[u8] = &[0x08, 0x00];
        let mut buf = [0u8; 2];
        let serializer = compression_method(&CompressionMethod::Deflate);
        let (_, _) = gen(serializer, &mut buf[..]).unwrap();
        assert_eq!(expected, buf);
    }
}
