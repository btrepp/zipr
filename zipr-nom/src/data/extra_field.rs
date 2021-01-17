use nom::{branch::alt, bytes::complete::take, combinator::map, IResult};
use zipr_data::borrowed::extra_field::ExtraField;

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

    use core::convert::TryInto;

    use zipr_data::{borrowed::extra_field::ntfs::NTFS};

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x63..0x87];
        let result = parse_extra_field(data);
        let expected = {
            let ntfs = NTFS {
                atime: 132514708162669827.try_into().unwrap(),
                mtime: 132514707831351075.try_into().unwrap(),
                ctime: 132514707783459448.try_into().unwrap(),
            };
            ExtraField::NTFS(ntfs)
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
