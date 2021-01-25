use crate::{
    error::{AppError, AppResult},
    sequence::Sequence,
};
use std::{
    convert::{TryFrom, TryInto},
    path::Path,
};
use zipr::{
    data::{
        borrowed::{file::CompressedData, OEM437Str, OEM437Symbols, ZipEntry},
        CompressionMethod, HostCompatibility, Version, ZipSpecification,
    },
    std::ToPath,
};

/// Adds files to an existing archive
pub fn add_files<P: AsRef<Path>>(
    file: P,
    files: Vec<P>,
    compression: CompressionMethod,
) -> AppResult<()> {
    fn to_zip<'a>(path: &'a Path, compressed_data: CompressedData<'a>) -> AppResult<ZipEntry<'a>> {
        let comment = Default::default();
        let extra_field = zipr::data::borrowed::extra_field::ExtraField::Unknown(&[]);
        let file_modification_time = zipr::data::DosTime::from_u16_unchecked(0);
        let file_modification_date = zipr::data::DosDate::from_u16_unchecked(0);
        let symbols = OEM437Symbols::try_from(path.to_str().unwrap())?;
        let str: OEM437Str = *symbols.as_ref();
        let file_name = zipr::data::borrowed::ZipPath::try_from(str)?;

        let version = Version {
            host: HostCompatibility::MSDOS,
            spec: ZipSpecification {
                major: 2u8.try_into().unwrap(),
                minor: 0u8.try_into().unwrap(),
            },
        };

        let entry = ZipEntry {
            version_made_by: version,
            version_needed: version,
            general_purpose: 0,
            file_modification_time,
            file_modification_date,
            internal_file_attributes: 0,
            external_file_attributes: 0,
            file_name,
            extra_field,
            comment,
            compressed_data,
        };
        Ok(entry)
    };

    let path = file.as_ref();
    let files: Vec<&Path> = files.iter().map(|x| x.as_ref()).collect();
    println!("{}", path.to_string_lossy());

    // Get the input bytes
    let bytes = if path.exists() {
        std::fs::read(path)?
    } else {
        Vec::new()
    };

    // At its core we need existing entries to determine what entries to store
    let entries = if !bytes.is_empty() {
        let entries = zipr::nom::iter::zip_entry_iter(&bytes)
            .sequence()
            .map_err(Into::<AppError>::into)?;
        entries
    } else {
        Vec::new()
    };

    // Filter out the entries that we already have
    let mut existing: Vec<_> = entries
        .into_iter()
        .filter(|x| !files.contains(&x.file_name.to_path().as_path()))
        .collect();

    let mut pool: Vec<Vec<u8>> = Vec::new();

    let mut new_entries = {
        let mut new_entries: Vec<ZipEntry> = Vec::new();
        for _ in files.iter() {
            pool.push(Vec::new())
        }

        for (i, buf) in pool.iter_mut().enumerate() {
            let f = std::fs::read(files[i]).unwrap();
            let compress = zipr::compression::compress_with(compression, buf, &f);
            let zip = to_zip(files[i], compress).unwrap();
            new_entries.push(zip)
        }
        new_entries
    };
    existing.append(&mut new_entries);

    let mut zip = std::fs::File::create(path)?;
    let serializer = zipr::cookie::file(existing.iter());
    let _ = cookie_factory::gen(serializer, &mut zip);

    Ok(())
}
