/// This is a wrapper type for a CP437 (OEM437) string
/// https://en.wikipedia.org/wiki/Code_page_437
/// Note any u8 is a valid character in this encoding
/// but it won't translate directly out to rust strings
/// you need conversion logic
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
pub struct OEM437Str<'a>(&'a [u8]);

impl<'a, T> From<&'a T> for OEM437Str<'a>
where
    T: AsRef<[u8]>,
{
    fn from(slice: &'a T) -> Self {
        OEM437Str(slice.as_ref())
    }
}

impl<'a> From<&'a [u8]> for OEM437Str<'a> {
    fn from(slice: &'a [u8]) -> Self {
        OEM437Str(slice)
    }
}

impl<'a> AsRef<[u8]> for OEM437Str<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

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
