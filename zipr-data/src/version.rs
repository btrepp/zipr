use core::convert::{TryFrom, TryInto};

use crate::util::{Bounded, OutOfRangeError};
use typenum::{U0, U1, U10, U25};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HostCompatibility {
    MSDOS,
    OpenVMS,
    Amiga,
    UNIX,
    VMCMS,
    AtariST,
    OS2,
    Macintosh,
    ZSystem,
    CPM,
    WindowsNTFS,
    MVS,
    VSE,
    AcornRisc,
    VFAT,
    AlternateMVS,
    BeOS,
    Tandem,
    OS400,
    OSX,
    Other(u8),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MinorVersion(Bounded<U0, U10>);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MajorVersion(Bounded<U1, U25>);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ZipSpecification {
    pub major: MajorVersion,
    pub minor: MinorVersion,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Version {
    pub host: HostCompatibility,
    pub spec: ZipSpecification,
}

impl From<ZipSpecification> for u8 {
    fn from(z: ZipSpecification) -> Self {
        let maj: u8 = z.major.0.into();
        let min: u8 = z.minor.0.into();
        (maj * 10) + min
    }
}

impl TryFrom<u8> for MajorVersion {
    type Error = OutOfRangeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let bounded = value.try_into()?;
        Ok(MajorVersion(bounded))
    }
}

impl TryFrom<u8> for MinorVersion {
    type Error = OutOfRangeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let bounded = value.try_into()?;
        Ok(MinorVersion(bounded))
    }
}
