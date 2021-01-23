/// This is a wrapper type for a CP437 (OEM437) string
/// https://en.wikipedia.org/wiki/Code_page_437
/// Note any u8 is a valid character in this encoding
/// but it won't translate directly out to rust strings
/// you need conversion logic
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
pub struct OEM437Str<'a>(&'a [u8]);

/// We can create the typed pointed for this str
/// from a slice of u8s. Note: it is expected
/// that the input is correctly mapped. Parsing say
/// utf8 strings here would mangle your text, but it would
/// be valid text
impl<'a> From<&'a [u8]> for OEM437Str<'a> {
    fn from(slice: &'a [u8]) -> Self {
        OEM437Str(slice)
    }
}

/// Also support anything that can be treated as a
/// ref of u8s. Typically this is handy
/// when we have 'static characters and allows construction
/// like OEM437Str::from(b"hello")
impl<'a, T> From<&'a T> for OEM437Str<'a>
where
    T: AsRef<[u8]>,
{
    fn from(slice: &'a T) -> Self {
        OEM437Str(slice.as_ref())
    }
}

/// Allows the string to be treated as a slice of u8
impl<'a> AsRef<[u8]> for OEM437Str<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

/// Allow the string to be treated as a slice of bytes
/// this is useful in the instances in which people want to
/// call information like oem437.len()
impl<'a> core::ops::Deref for OEM437Str<'a> {
    type Target = [u8];

    fn deref(&self) -> &'a Self::Target {
        self.0
    }
}

mod symbols;
pub use symbols::*;

#[cfg(test)]
mod tests {}
