use cookie_factory::lib::std::io::Write;
use cookie_factory::{combinator::slice, SerializeFn};
use zipr_core::data::ZipPath;

pub fn zip_path<'a, W: 'a>(characters: &'a ZipPath<'a>) -> impl SerializeFn<W> + 'a
where
    W: Write,
{
    let bytes = characters.to_bytes();
    slice(bytes)
}
