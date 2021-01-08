use nom::{branch::alt, bytes::complete::take, combinator::map, IResult};

use zipr_core::data::extra_field::ExtraField;

use super::ntfs::parse_ntfs;

/// Uses alternatives to try and pass the extra field.
/// Will return one of the datatypes, or fail
pub fn parse_extra_field(input: &[u8]) -> IResult<&[u8], ExtraField<'_>> {
    let ntfs = map(parse_ntfs, ExtraField::NTFS);
    let unknown = {
        let len = input.len();
        map(take(len), ExtraField::Unknown)
    };
    let (input, result) = alt((ntfs, unknown))(input)?;

    Ok((input, result))
}

#[cfg(test)]
mod tests {

    use zipr_core::data::extra_field::{ntfs::NTFS, wintimestamp::WinTimestamp};

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x63..0x87];
        let result = parse_extra_field(data);
        let expected = {
            let ntfs = NTFS {
                atime: WinTimestamp::from_u64_unchecked(132514708162669827),
                mtime: WinTimestamp::from_u64_unchecked(132514707831351075),
                ctime: WinTimestamp::from_u64_unchecked(132514707783459448),
            };
            ExtraField::NTFS(ntfs)
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
