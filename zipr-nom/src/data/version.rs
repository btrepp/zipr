use core::convert::TryInto;

use nom::{number::complete::le_u8, IResult};
use zipr_data::{HostCompatibility, Version, ZipSpecification};

fn hostcompat(input: u8) -> HostCompatibility {
    match input {
        0 => HostCompatibility::MSDOS,
        1 => HostCompatibility::Amiga,
        2 => HostCompatibility::OpenVMS,
        3 => HostCompatibility::UNIX,
        4 => HostCompatibility::VMCMS,
        5 => HostCompatibility::AtariST,
        6 => HostCompatibility::OS2,
        7 => HostCompatibility::Macintosh,
        8 => HostCompatibility::ZSystem,
        9 => HostCompatibility::CPM,
        10 => HostCompatibility::WindowsNTFS,
        11 => HostCompatibility::MVS,
        12 => HostCompatibility::VSE,
        13 => HostCompatibility::AcornRisc,
        14 => HostCompatibility::VFAT,
        15 => HostCompatibility::AlternateMVS,
        16 => HostCompatibility::BeOS,
        17 => HostCompatibility::Tandem,
        18 => HostCompatibility::OS400,
        19 => HostCompatibility::OSX,
        x => HostCompatibility::Other(x),
    }
}
/// Uses alternatives to try and pass the extra field.
/// Will return one of the datatypes, or fail
pub fn parse_version(input: &[u8]) -> IResult<&[u8], Version> {
    let (input, version) = le_u8(input)?;
    let (input, host) = le_u8(input)?;
    let major = (version / 10).try_into().unwrap();
    let minor = (version % 10).try_into().unwrap();
    let spec = ZipSpecification { major, minor };
    let host = hostcompat(host);
    let version = Version { host, spec };

    Ok((input, version))
}

#[cfg(test)]
mod tests {}
