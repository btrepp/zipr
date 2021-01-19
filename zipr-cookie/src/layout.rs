use zipr_data::{
    borrowed::{
        file::{CentralDirectoryEntry, EndOfCentralDirectory, LocalFileEntry},
        ZipEntry,
    },
    constants,
};

use zipr_domain::zip_entry_to_files;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ZipPart<'a> {
    LocalFile(LocalFileEntry<'a>),
    DirectoryEntry(CentralDirectoryEntry<'a>),
    EndOfCentralDirectory(EndOfCentralDirectory<'a>),
}

#[derive(PartialEq, Clone, Copy)]
enum State {
    LocalFiles,
    Directory,
    Eocd,
    Done,
}

#[derive(Clone)]
pub struct LayoutIterator<'a, I>
where
    I: Iterator<Item = &'a ZipEntry<'a>>,
{
    items: u16,
    position: u32,
    state: State,
    first_pass: I,
    second_pass: I,
    comment: &'a ascii::AsciiStr,
    size_of_directory: u32,
}

impl<'a, I> Iterator for LayoutIterator<'a, I>
where
    I: Iterator<Item = &'a ZipEntry<'a>>,
{
    type Item = ZipPart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let state = &mut self.state;
        // If in local files. Just emit the structure
        if state == &State::LocalFiles {
            match self.first_pass.next() {
                None => {
                    *state = State::Directory;
                }
                Some(x) => {
                    let (local, _) = zip_entry_to_files(0, x);
                    return Some(ZipPart::LocalFile(local));
                }
            }
        };

        // In directory emmitting mode
        if state == &State::Directory {
            match self.second_pass.next() {
                None => *state = State::Eocd,
                Some(x) => {
                    let (_, directory) = zip_entry_to_files(self.position, x);
                    let increment = constants::LOCAL_FILE_MIN_LENGTH as u32
                        + directory.file_name.len() as u32
                        + directory.compressed_size as u32
                        + directory.extra_field.serialized_len() as u32;
                    let directory_increment = constants::CENTRAL_DIRECTORY_HEAD_MIN_LENGTH as u32
                        + directory.file_name.len() as u32
                        + directory.extra_field.serialized_len() as u32
                        + directory.comment.len() as u32;

                    self.position += increment;
                    self.size_of_directory += directory_increment;
                    self.items += 1;

                    return Some(ZipPart::DirectoryEntry(directory));
                }
            }
        };

        // Finally write a single Eocd record out
        if state == &State::Eocd {
            let eocd = EndOfCentralDirectory {
                total_number_records: self.items,
                size_of_directory: self.size_of_directory,
                offset_start_directory: self.position,
                comment: self.comment,
            };
            *state = State::Done;
            return Some(ZipPart::EndOfCentralDirectory(eocd));
        }

        // Else we have nothing more for you
        None
    }
}

/// Creates a iterator that returns the parts in order to be serialized
/// Note this is O(2N), as we need to run through the list twice
pub fn layout<'a, I>(data: I) -> LayoutIterator<'a, I>
where
    I: Iterator<Item = &'a ZipEntry<'a>> + Clone,
{
    let first_pass = data.clone();
    let second_pass = data;
    let state = State::LocalFiles;
    LayoutIterator {
        items: 0,
        position: 0,
        first_pass,
        second_pass,
        state,
        size_of_directory: 0,
        comment: ascii::AsciiStr::from_ascii(b"").unwrap(),
    }
}

#[cfg(test)]
mod tests {

    use core::{convert::TryInto, panic};
    use zipr_data::{
        borrowed::{extra_field::ExtraField, file::CompressedData, ZipEntry, ZipPath},
        constants, CompressionMethod, DosDate, DosTime, HostCompatibility, Version,
        ZipSpecification,
    };

    use super::{layout, ZipPart};

    fn single_entry() -> ZipEntry<'static> {
        let bytes = b"hello";
        let compressed_data =
            CompressedData::create_unchecked(5, CompressionMethod::Stored, 0x3610A686, bytes);

        let input = ZipEntry {
            version_made_by: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 6u8.try_into().unwrap(),
                    minor: 3u8.try_into().unwrap(),
                },
            },
            version_needed: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 1u8.try_into().unwrap(),
                    minor: 0u8.try_into().unwrap(),
                },
            },
            general_purpose: 0,
            file_modification_date: DosDate::from_u16_unchecked(0),
            file_modification_time: DosTime::from_u16_unchecked(0),
            file_name: ZipPath::create_from_string(
                ascii::AsciiStr::from_ascii(b"hello.txt").unwrap(),
            )
            .unwrap(),
            external_file_attributes: 0,
            internal_file_attributes: 0,
            extra_field: ExtraField::Unknown(&[]),
            comment: ascii::AsciiStr::from_ascii(b"").unwrap(),
            compressed_data,
        };
        input
    }

    fn two_entries() -> [ZipEntry<'static>; 2] {
        let a = single_entry();
        let mut b = single_entry();
        b.file_name =
            ZipPath::create_from_string(ascii::AsciiStr::from_ascii(b"second").unwrap()).unwrap();
        [a, b]
    }

    #[test]
    fn localfile_is_first() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.next().unwrap();

        match first_item {
            ZipPart::LocalFile(_) => (),
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn directory_is_third() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(2).unwrap();

        match first_item {
            ZipPart::DirectoryEntry(_) => (),
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn directory_is_fourth() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(3).unwrap();

        match first_item {
            ZipPart::DirectoryEntry(_) => (),
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn eocd_is_fifth() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(4).unwrap();

        match first_item {
            ZipPart::EndOfCentralDirectory(_) => (),
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn ends_after_eocd() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(5);

        assert_eq!(None, first_item);
    }

    #[test]
    fn directory_is_third_name() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(2).unwrap();

        match first_item {
            ZipPart::DirectoryEntry(x) => {
                assert_eq!(x.file_name, input[0].file_name)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn directory_is_fourth_name() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(3).unwrap();

        match first_item {
            ZipPart::DirectoryEntry(x) => {
                assert_eq!(x.file_name, input[1].file_name)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn directory_is_third_offset() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(2).unwrap();

        match first_item {
            ZipPart::DirectoryEntry(x) => {
                assert_eq!(0, x.relative_offset)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn directory_is_fourth_offset() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(3).unwrap();

        let file_name_length = input[0].file_name.len() as u32;
        let data_length = input[0].compressed_data.bytes().len() as u32;
        let expected_position =
            file_name_length + data_length + constants::LOCAL_FILE_MIN_LENGTH as u32;
        match first_item {
            ZipPart::DirectoryEntry(x) => {
                assert_eq!(expected_position, x.relative_offset)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn eocd_is_fifth_directory_position() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(4).unwrap();

        let file_name_length_first = input[0].file_name.len() as u32;
        let data_length_first = input[0].compressed_data.bytes().len() as u32;
        let expected_position_first =
            file_name_length_first + data_length_first + constants::LOCAL_FILE_MIN_LENGTH as u32;

        let file_name_length_second = input[1].file_name.len() as u32;
        let data_length_second = input[1].compressed_data.bytes().len() as u32;
        let expected_position_second =
            file_name_length_second + data_length_second + constants::LOCAL_FILE_MIN_LENGTH as u32;

        let expected_position = expected_position_first + expected_position_second;
        match first_item {
            ZipPart::EndOfCentralDirectory(x) => {
                assert_eq!(expected_position, x.offset_start_directory)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn eocd_is_fifth_size() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(4).unwrap();
        let file_names = 9 + 6_u32;
        let size = (2 * constants::CENTRAL_DIRECTORY_HEAD_MIN_LENGTH as u32) + file_names;
        match first_item {
            ZipPart::EndOfCentralDirectory(x) => {
                assert_eq!(size as u32, x.size_of_directory)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn eocd_is_fifth_start() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(4).unwrap();

        let start_of_directory = 85_u32;

        match first_item {
            ZipPart::EndOfCentralDirectory(x) => {
                assert_eq!(start_of_directory, x.offset_start_directory)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }

    #[test]
    fn eocd_is_fifth_count() {
        let input = two_entries();
        let mut result = layout(input.iter());
        let first_item = result.nth(4).unwrap();

        match first_item {
            ZipPart::EndOfCentralDirectory(x) => {
                assert_eq!(2, x.total_number_records)
            }
            x => panic!("Expected Localfileentry: {:?}", x),
        }
    }
}
