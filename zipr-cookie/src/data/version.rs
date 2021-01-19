use cookie_factory::{combinator::slice, lib::std::io::Write, SerializeFn};
use zipr_data::{HostCompatibility, Version};

fn host(host: HostCompatibility) -> u8 {
    match host {
        HostCompatibility::MSDOS => 0,
        HostCompatibility::Amiga => 1,
        HostCompatibility::OpenVMS => 2,
        HostCompatibility::UNIX => 3,
        HostCompatibility::VMCMS => 4,
        HostCompatibility::AtariST => 5,
        HostCompatibility::OS2 => 6,
        HostCompatibility::Macintosh => 7,
        HostCompatibility::ZSystem => 8,
        HostCompatibility::CPM => 9,
        HostCompatibility::WindowsNTFS => 10,
        HostCompatibility::MVS => 11,
        HostCompatibility::VSE => 12,
        HostCompatibility::AcornRisc => 13,
        HostCompatibility::VFAT => 14,
        HostCompatibility::AlternateMVS => 15,
        HostCompatibility::BeOS => 16,
        HostCompatibility::Tandem => 17,
        HostCompatibility::OS400 => 18,
        HostCompatibility::OSX => 19,
        HostCompatibility::Other(x) => x,
    }
}

pub fn version<W>(version: Version) -> impl SerializeFn<W>
where
    W: Write,
{
    let upper = host(version.host);
    let lower: u8 = version.spec.into();
    slice([lower, upper])
}
