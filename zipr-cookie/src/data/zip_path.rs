use cookie_factory::SerializeFn;
use cookie_factory::{combinator::slice, lib::std::io::Write, WriteContext};
use zipr_data::borrowed::ZipPath;

pub fn zip_path<'a, W: 'a>(characters: &'a ZipPath<'a>) -> impl SerializeFn<W> + 'a
where
    W: Write,
{
    let cp437 = characters.to_cp437();
    move |out: WriteContext<W>| {
        let characters = cp437.as_slice();
        slice(characters)(out)
    }
}
