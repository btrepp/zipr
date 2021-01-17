use cookie_factory::{lib::std::io::Write, multi::all, SerializeFn, WriteContext};
use zipr_data::borrowed::ZipEntry;

use crate::{
    data::{central_directory_entry, end_of_central_directory, local_file_entry},
    layout::{layout, ZipPart},
};

fn parts<'a, W: Write + 'a>(input: ZipPart<'a>) -> impl SerializeFn<W> + 'a {
    move |out: WriteContext<W>| match input {
        ZipPart::LocalFile(x) => local_file_entry(&x)(out),
        ZipPart::DirectoryEntry(x) => central_directory_entry(&x)(out),
        ZipPart::EndOfCentralDirectory(x) => end_of_central_directory(&x)(out),
    }
}

/// A serializer that takes an iterator of zip entries
///
/// This will enumerate the list twice, and layout all the
/// datastructures correctly. It then adds the eocd when complete
pub fn file<'a, W: Write + 'a, I>(input: I) -> impl SerializeFn<W> + 'a
where
    I: Iterator<Item = &'a ZipEntry<'a>> + Clone + 'a,
    W: Write + 'a,
{
    let layout = layout(input).map(parts);
    all(layout)
}
