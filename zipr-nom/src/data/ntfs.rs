use core::convert::TryInto;

use nom::{
    bytes::complete::tag, combinator::map_res, number::complete::le_u32, number::complete::le_u64,
    IResult,
};
use zipr_data::{
    borrowed::extra_field::ntfs::NTFS, constants::EXTRA_FIELD_NTFS_HEADER, WinTimestamp,
};

pub fn parse_ntfs(input: &[u8]) -> IResult<&[u8], NTFS> {
    let (input, _) = tag(EXTRA_FIELD_NTFS_HEADER)(input)?;
    // tsize is static currently
    let (input, _tsize) = tag([0x20, 0x00])(input)?;
    let (input, _reserved) = le_u32(input)?;

    let (input, _tag1) = tag([0x1, 0x0])(input)?;
    let (input, _size1) = tag([0x18, 0])(input)?;

    let (input, mtime) = map_res(le_u64, TryInto::<WinTimestamp>::try_into)(input)?;
    let (input, atime) = map_res(le_u64, TryInto::<WinTimestamp>::try_into)(input)?;
    let (input, ctime) = map_res(le_u64, TryInto::<WinTimestamp>::try_into)(input)?;

    let result = NTFS {
        mtime,
        ctime,
        atime,
    };

    Ok((input, result))
}

#[cfg(test)]
mod tests {

    use core::convert::TryInto;

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x63..0x87];
        let result = parse_ntfs(data);
        let expected = NTFS {
            atime: 132514708162669827.try_into().unwrap(),
            mtime: 132514707831351075.try_into().unwrap(),
            ctime: 132514707783459448.try_into().unwrap(),
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
