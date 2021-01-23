use cookie_factory::SerializeFn;
use cookie_factory::{combinator::slice, lib::std::io::Write};
use zipr_data::borrowed::ZipPath;

pub fn zip_path<'a, W: 'a>(characters: &'a ZipPath<'a>) -> impl SerializeFn<W> + 'a
where
    W: Write,
{
    slice(characters.as_ref())
}
