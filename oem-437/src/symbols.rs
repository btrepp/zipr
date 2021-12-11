use crate::{
    symbolmap::{char_to_oem437, oem437_lookup_unicode_char},
    OEM437Str,
};
use core::fmt::Write;
use core::str::from_utf8;
use std::convert::TryFrom;
#[derive(PartialEq, Copy, Clone)]

/// New type in which we are happy to treat all characters as symbols
/// some strategies treat certain values as control characters instead
pub struct OEM437Symbols<'a>(OEM437Str<'a>);

/// This is a trivial newtype over the oemstr, so just wrap it
impl<'a> From<OEM437Str<'a>> for OEM437Symbols<'a> {
    fn from(str: OEM437Str<'a>) -> Self {
        OEM437Symbols(str)
    }
}

#[derive(Debug)]
pub struct NotValidOEM437();
impl<'a> TryFrom<&'a str> for OEM437Symbols<'a> {
    type Error = NotValidOEM437;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let is_safe = value.chars().all(|x| char_to_oem437(x).is_some());
        if is_safe {
            let bytes = value.as_bytes();
            let str = OEM437Str(bytes);
            let symbol = OEM437Symbols(str);
            Ok(symbol)
        } else {
            Err(NotValidOEM437())
        }
    }
}

/// Allows the newtype to be treated as a unknown str
/// Helpful for traits
impl<'a> AsRef<OEM437Str<'a>> for OEM437Symbols<'a> {
    fn as_ref(&self) -> &OEM437Str<'a> {
        &self.0
    }
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
            oem437_lookup_unicode_char(*byte)
        }

        self.data.next().map(expand)
    }
}

/// Try and cast the OEM437Str as-is to
/// a str. Will fail if there is a character
/// that doesn't map in the same space
pub trait AsSymbols<'a> {
    /// Casts the block as-is if compatible
    /// certain subsets of UTF8 match OEM437, so if
    /// you restrict yourself to these characters it
    /// is convertable without re-allocing
    fn as_utf8(&self) -> Option<&'a str>;

    /// In cases where as_utf8 is not possible.
    /// This allows us to 'expand' the string to utf8
    /// as some of the symbols in oem437 may be multibyte
    /// The iterator allows you to get the chars without allocating.
    /// if you wish to have a String (and have an allocator) call
    /// oem437.to_utf8().collect::<String>()
    fn to_utf8(&'a self) -> SymbolIterator<'a>;
}

impl<'a, T> AsSymbols<'a> for T
where
    T: AsRef<OEM437Str<'a>>,
{
    fn as_utf8(&self) -> Option<&'a str> {
        // There is probably more scope here.
        // its just checking ascii, and i'm unsure if the
        // values are mapped correctly from_utf8?
        fn convertable(byte: &u8) -> bool {
            *byte > 32 && *byte < 127
        }
        let bytes = self.as_ref().0;
        let valid = bytes.iter().all(convertable);

        if valid {
            from_utf8(bytes).ok()
        } else {
            None
        }
    }

    fn to_utf8(&'a self) -> SymbolIterator<'a> {
        let iter = self.as_ref().as_ref().iter();
        SymbolIterator { data: iter }
    }
}

/// Display is only implemented when we are happy with it
/// being a symbol
impl<'a> core::fmt::Display for OEM437Symbols<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.to_utf8() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
