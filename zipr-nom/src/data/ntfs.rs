use nom::{
    bytes::complete::tag, combinator::map, number::complete::le_u32, number::complete::le_u64,
    IResult,
};
use zipr_core::{
    constants::EXTRA_FIELD_NTFS_HEADER,
    data::extra_field::{ntfs::NTFS, wintimestamp::WinTimestamp},
};

pub fn parse_ntfs(input: &[u8]) -> IResult<&[u8], NTFS> {
    let (input, _) = tag(EXTRA_FIELD_NTFS_HEADER)(input)?;
    // tsize is static currently
    let (input, _tsize) = tag([0x20, 0x00])(input)?;
    let (input, _reserved) = le_u32(input)?;

    let (input, _tag1) = tag([0x1, 0x0])(input)?;
    let (input, _size1) = tag([0x18, 0])(input)?;

    let (input, mtime) = map(le_u64, WinTimestamp::from_u64_unchecked)(input)?;
    let (input, atime) = map(le_u64, WinTimestamp::from_u64_unchecked)(input)?;
    let (input, ctime) = map(le_u64, WinTimestamp::from_u64_unchecked)(input)?;

    let result = NTFS {
        mtime,
        ctime,
        atime,
    };

    Ok((input, result))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x63..0x87];
        let result = parse_ntfs(data);
        let expected = NTFS {
            atime: WinTimestamp::from_u64_unchecked(132514708162669827),
            mtime: WinTimestamp::from_u64_unchecked(132514707831351075),
            ctime: WinTimestamp::from_u64_unchecked(132514707783459448),
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
