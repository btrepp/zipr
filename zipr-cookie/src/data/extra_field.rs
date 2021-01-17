use cookie_factory::{
    bytes::{le_u16, le_u32, le_u64},
    combinator::slice,
    lib::std::io::Write,
    sequence::tuple,
    SerializeFn, WriteContext,
};
use wintimestamp::WinTimestamp;
use zipr_data::{
    borrowed::extra_field::{ntfs::NTFS, wintimestamp, ExtraField},
    constants::{self, EXTRA_FIELD_NTFS_HEADER},
};

pub fn extra_field<'a, W: Write + 'a>(input: ExtraField<'a>) -> impl SerializeFn<W> + 'a {
    move |out: WriteContext<W>| match input {
        ExtraField::NTFS(n) => ntfs(&n)(out),
        ExtraField::Unknown(b) => unknown(b)(out),
    }
}

pub fn extra_field_len<'a, W: Write + 'a>(input: &ExtraField<'a>) -> impl SerializeFn<W> + 'a {
    let len = match *input {
        ExtraField::NTFS(_) => constants::EXTRA_FIELD_NTFS_LENGTH as u16,
        ExtraField::Unknown(b) => b.len() as u16,
    };
    le_u16(len)
}

pub fn unknown<'a, W: Write + 'a>(input: &'a [u8]) -> impl SerializeFn<W> + 'a {
    tuple((slice(input),))
}
pub fn ntfs<'a, W: Write + 'a>(input: &NTFS) -> impl SerializeFn<W> + 'a {
    tuple((
        slice(EXTRA_FIELD_NTFS_HEADER),
        slice(&[0x20, 0x00]),
        le_u32(0),
        slice(&[0x01, 0x00]),
        slice(&[0x18, 0x00]),
        wintimestamp(&input.mtime),
        wintimestamp(&input.atime),
        wintimestamp(&input.ctime),
    ))
}

pub fn wintimestamp<W: Write>(input: &WinTimestamp) -> impl SerializeFn<W> {
    le_u64(input.as_u64())
}
