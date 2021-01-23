use crate::OEM437Str;
use core::fmt::Write;
use core::str::{from_utf8, Utf8Error};

#[derive(PartialEq, Copy, Clone)]
/// Treat the string as symbols
pub struct OEM437Symbols<'a>(OEM437Str<'a>);

/// Occurs when we are unable to translate
#[derive(Debug)]
pub enum OEM437ToUtf8Error {
    StringWouldExpand,
    Utf8Error(Utf8Error),
}

/// An iterator over the OEM437 that will yield the
/// next character. This allows it to 'expand'
/// into whatever form is required in the program
pub struct SymbolIterator<'a> {
    data: core::slice::Iter<'a, u8>,
}

impl<'a> core::iter::Iterator for SymbolIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        fn expand(byte: &u8) -> char {
            let u32_val = *byte as u32; //Needs a lookup
            core::char::from_u32(u32_val).unwrap() //? should be safe from our mapping
        }

        self.data.next().map(expand)
    }
}

impl<'a> From<&OEM437Str<'a>> for OEM437Symbols<'a> {
    fn from(str: &OEM437Str<'a>) -> Self {
        OEM437Symbols(*str)
    }
}

/// Try and cast the OEM437Str as-is to
/// a str. Will fail if there is a character
/// that doesn't map in the same space
pub trait AsSymbols<'a> {
    fn as_utf8(self) -> Result<&'a str, OEM437ToUtf8Error>;
    fn to_utf8(&self) -> SymbolIterator<'a>;
}

impl<'a> AsSymbols<'a> for OEM437Str<'a> {
    fn as_utf8(self) -> Result<&'a str, OEM437ToUtf8Error> {
        let symbols: OEM437Symbols<'a> = OEM437Symbols(self);
        symbols.as_utf8()
    }

    fn to_utf8(&self) -> SymbolIterator<'a> {
        let symbols = OEM437Symbols(*self);
        symbols.to_utf8()
    }
}

impl<'a> AsSymbols<'a> for OEM437Symbols<'a> {
    fn as_utf8(self) -> Result<&'a str, OEM437ToUtf8Error> {
        // There is probably more scope here.
        // its just checking ascii, and i'm unsure if the
        // values are mapped correctly from_utf8?
        fn convertable(byte: &u8) -> bool {
            *byte > 32 && *byte < 127
        }
        let bytes = self.0 .0;
        let valid = bytes.iter().all(convertable);

        if valid {
            from_utf8(bytes).map_err(OEM437ToUtf8Error::Utf8Error)
        } else {
            Err(OEM437ToUtf8Error::StringWouldExpand)
        }
    }

    fn to_utf8(&self) -> SymbolIterator<'a> {
        let slice = self.0;
        let iter = slice.as_slice().iter();
        SymbolIterator { data: iter }
    }
}

impl<'a> core::fmt::Display for OEM437Symbols<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.to_utf8() {
            f.write_char(c)?;
        }
        Ok(())
    }
}
