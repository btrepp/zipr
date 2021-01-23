use core::convert::TryFrom;
use oem_437::OEM437Str;

/// A borrow of a ascii str
///
/// This should represent a valid path for entries in the
/// zip file. Note this is a subset of path or asciistr.
/// as there are invalid states possible
///
/// The name of the file, with optional relative path.
/// The path stored MUST NOT contain a drive or
/// device letter, or a leading slash.  All slashes
/// MUST be forward slashes '/' as opposed to
/// backwards slashes '\' for compatibility with Amiga
/// and UNIX file systems etc.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ZipPath<'a>(OEM437Str<'a>);

#[derive(Debug)]
pub enum ZipPathError {}

/// Creates a the zippath from a oem437string
impl<'a> TryFrom<OEM437Str<'a>> for ZipPath<'a> {
    type Error = ZipPathError;

    fn try_from(value: OEM437Str<'a>) -> Result<Self, Self::Error> {
        Ok(ZipPath(value))
    }
}

/// Helper implementation as most people usually will want to use u8s
/// For specific cases of b"hello" this is predictable.
/// For b'unicodecharcters' things will not be as nice
impl<'a> TryFrom<&'a [u8]> for ZipPath<'a> {
    type Error = ZipPathError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let bytes = OEM437Str::from(value);
        ZipPath::try_from(bytes)
    }
}

/// Allow zippath to be passed as a oem437 str
/// Mainly handy in printing
impl<'a> AsRef<OEM437Str<'a>> for ZipPath<'a> {
    fn as_ref(&self) -> &OEM437Str<'a> {
        &self.0
    }
}

/*
impl<'a> ZipPath<'a> {
   pub fn from_cp437(string: OEM437Str<'a>) -> Result<Self, ZipPathError> {
        // need to validate this in future
        Ok(ZipPath(string))
    }

    pub fn to_cp437(&self) -> OEM437Str<'a> {
        self.0
    }
}
 */
