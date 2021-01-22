/// This is a wrapper type for a CP437 (OEM437) string
/// https://en.wikipedia.org/wiki/Code_page_437
/// Note any u8 is a valid character in this encoding
/// but it won't translate directly out to rust strings
/// you need conversion logic
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
pub struct OEM437Str<'a>(&'a [u8]);

// We keep the implmentation incredibly bare-bones
// as there are a few different ways of translating this
impl<'a> OEM437Str<'a> {
    pub fn from_slice(str: &[u8]) -> OEM437Str<'_> {
        OEM437Str(str)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}
